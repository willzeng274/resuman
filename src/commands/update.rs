use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use clap::Parser;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::{fs, path::PathBuf};

use crate::config::Config;

#[derive(Parser, Debug)]
#[command(name = "update", about = "Update resume metadata", visible_aliases = ["edit", "modify", "change", "alter", "revise", "amend", "correct", "fix", "adjust", "tweak"])]
pub struct UpdateCommand {
    #[arg(index = 1, help = "ID of the resume to update")]
    pub id: i32,

    #[arg(short, long, help = "Update company name (metadata only)")]
    pub company: Option<String>,

    #[arg(short, long, help = "Update group name (metadata only)")]
    pub group: Option<String>,

    #[arg(short, long, help = "Update template used (metadata only)")]
    pub template: Option<String>,

    #[arg(
        short = 'f',
        long,
        help = "Update \"copied from\" (metadata only)",
        visible_aliases = ["tex", "copy"]
    )]
    pub file: Option<PathBuf>,

    #[arg(short, long, help = "Update position/role (metadata only)", visible_aliases = ["role", "job"])]
    pub position: Option<String>,

    #[arg(
        short = 'a',
        long = "letter",
        help = "Update cover letter status (metadata only)"
    )]
    pub has_cover_letter: Option<bool>,

    #[arg(
        long,
        help = "Update created date (metadata only)",
        visible_aliases = ["created"]
    )]
    pub created_at: Option<DateTime<Utc>>,

    #[arg(long, help = "Update date applied (metadata)", visible_aliases = ["applied", "applied_at"])]
    pub applied_time: Option<DateTime<Utc>>,

    #[arg(short = 'd', long, help = "Update length of job (metadata)", visible_aliases = ["duration"])]
    pub length: Option<u16>,

    #[arg(short, long, help = "Update location of job (metadata)")]
    pub location: Option<String>,

    #[arg(short, long, help = "Update status of application (metadata)")]
    pub status: Option<String>,

    #[arg(
        short,
        long,
        help = "Update URLs to job posting, company, etc (metadata)"
    )]
    pub urls: Option<String>,

    #[arg(short, long, help = "Update other metadata")]
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ResumeMetadata {
    applied_time: Option<i64>,
    company: Option<String>,
    copied_from: Option<String>,
    created_at: Option<i64>,
    group: Option<String>,
    has_cover_letter: Option<bool>,
    length: Option<u16>,
    location: Option<String>,
    notes: Option<String>,
    position: Option<String>,
    status: Option<String>,
    template: Option<String>,
    urls: Option<String>,
}

impl From<&UpdateCommand> for ResumeMetadata {
    fn from(cmd: &UpdateCommand) -> Self {
        ResumeMetadata {
            company: cmd.company.clone(),
            group: cmd.group.clone(),
            template: cmd.template.clone(),
            copied_from: cmd.file.as_ref().map(|p| p.to_string_lossy().into_owned()),
            position: cmd.position.clone(),
            has_cover_letter: cmd.has_cover_letter,
            created_at: cmd.created_at.map(|dt| dt.timestamp()),
            applied_time: cmd.applied_time.map(|dt| dt.timestamp()),
            length: cmd.length,
            location: cmd.location.clone(),
            status: cmd.status.clone(),
            urls: cmd.urls.clone(),
            notes: cmd.notes.clone(),
        }
    }
}

enum SqlParam<'a> {
    String(&'a Option<String>),
    Bool(&'a Option<bool>),
    DateTime(&'a Option<DateTime<Utc>>),
    U16(&'a Option<u16>),
}

impl<'a> SqlParam<'a> {
    fn bind_to_query(
        self,
        query: sqlx::query::Query<'a, sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'a>>,
    ) -> sqlx::query::Query<'a, sqlx::Sqlite, sqlx::sqlite::SqliteArguments<'a>> {
        match self {
            SqlParam::String(val) => query.bind(val.as_deref()),
            SqlParam::Bool(val) => query.bind(val),
            SqlParam::DateTime(val) => query.bind(val.map(|dt| dt.naive_utc())),
            SqlParam::U16(val) => query.bind(val.map(|v| v as i64)), // SQLite expects i64
        }
    }
}

pub async fn execute(_cfg: Config, args: &UpdateCommand, pool: &SqlitePool) -> Result<()> {
    log::debug!("Updating with arguments:\n{:#?}", args);

    // Fetch existing resume
    let resume = sqlx::query!(
        "SELECT r.*, m.metadata_file_path FROM resumes r JOIN metadata m ON r.id = m.resume_id WHERE r.id = ?1",
        args.id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| anyhow!("No resume found with id {}", args.id))?;

    // Read and update JSON file
    let file_path = &resume.metadata_file_path;
    let mut current_metadata: ResumeMetadata =
        serde_json::from_str(&fs::read_to_string(file_path)?)?;
    let update_metadata = ResumeMetadata::from(args);

    // Update JSON fields (same as before)
    if update_metadata.company.is_some() {
        current_metadata.company = update_metadata.company;
    }
    if update_metadata.group.is_some() {
        current_metadata.group = update_metadata.group;
    }
    if update_metadata.template.is_some() {
        current_metadata.template = update_metadata.template;
    }
    if update_metadata.copied_from.is_some() {
        current_metadata.copied_from = update_metadata.copied_from;
    }
    if update_metadata.position.is_some() {
        current_metadata.position = update_metadata.position;
    }
    if update_metadata.has_cover_letter.is_some() {
        current_metadata.has_cover_letter = update_metadata.has_cover_letter;
    }
    if update_metadata.created_at.is_some() {
        current_metadata.created_at = update_metadata.created_at;
    }
    if update_metadata.applied_time.is_some() {
        current_metadata.applied_time = update_metadata.applied_time;
    }
    if update_metadata.length.is_some() {
        current_metadata.length = update_metadata.length;
    }
    if update_metadata.location.is_some() {
        current_metadata.location = update_metadata.location;
    }
    if update_metadata.status.is_some() {
        current_metadata.status = update_metadata.status;
    }
    if update_metadata.urls.is_some() {
        current_metadata.urls = update_metadata.urls;
    }
    if update_metadata.notes.is_some() {
        current_metadata.notes = update_metadata.notes;
    }

    // Write updated metadata back to file
    fs::write(file_path, serde_json::to_string_pretty(&current_metadata)?)?;

    // Split updates between resumes and metadata tables
    let mut resumes_query_parts = Vec::new();
    let mut resumes_params: Vec<SqlParam> = Vec::new();

    let mut metadata_query_parts = Vec::new();
    let mut metadata_params: Vec<SqlParam> = Vec::new();

    // Fields for resumes table
    if args.company.is_some() {
        resumes_query_parts.push("company = ?");
        resumes_params.push(SqlParam::String(&args.company));
    }
    if args.group.is_some() {
        resumes_query_parts.push("\"group\" = ?");
        resumes_params.push(SqlParam::String(&args.group));
    }
    if args.template.is_some() {
        resumes_query_parts.push("template = ?");
        resumes_params.push(SqlParam::String(&args.template));
    }
    if args.position.is_some() {
        resumes_query_parts.push("position = ?");
        resumes_params.push(SqlParam::String(&args.position));
    }
    if args.has_cover_letter.is_some() {
        resumes_query_parts.push("has_cover_letter = ?");
        resumes_params.push(SqlParam::Bool(&args.has_cover_letter));
    }

    // Fields for metadata table
    let copied_from = args.file.as_ref().map(|p| p.to_string_lossy().into_owned());
    if args.file.is_some() {
        metadata_query_parts.push("copied_from = ?");
        metadata_params.push(SqlParam::String(&copied_from));
    }
    if args.applied_time.is_some() {
        metadata_query_parts.push("applied_time = ?");
        metadata_params.push(SqlParam::DateTime(&args.applied_time));
    }
    if args.length.is_some() {
        metadata_query_parts.push("length = ?");
        metadata_params.push(SqlParam::U16(&args.length));
    }
    if args.location.is_some() {
        metadata_query_parts.push("location = ?");
        metadata_params.push(SqlParam::String(&args.location));
    }
    if args.status.is_some() {
        metadata_query_parts.push("status = ?");
        metadata_params.push(SqlParam::String(&args.status));
    }
    if args.urls.is_some() {
        metadata_query_parts.push("urls = ?");
        metadata_params.push(SqlParam::String(&args.urls));
    }
    if args.notes.is_some() {
        metadata_query_parts.push("notes = ?");
        metadata_params.push(SqlParam::String(&args.notes));
    }

    // Execute updates for resumes table if needed
    if !resumes_query_parts.is_empty() {
        let query = format!(
            "UPDATE resumes SET {} WHERE id = ?",
            resumes_query_parts.join(", ")
        );

        let mut query = sqlx::query(&query);
        for param in resumes_params {
            query = param.bind_to_query(query);
        }
        query = query.bind(args.id);

        query.execute(pool).await?;
    }

    // Execute updates for metadata table if needed
    if !metadata_query_parts.is_empty() {
        let query = format!(
            "UPDATE metadata SET {} WHERE resume_id = ?",
            metadata_query_parts.join(", ")
        );

        let mut query = sqlx::query(&query);
        for param in metadata_params {
            query = param.bind_to_query(query);
        }
        query = query.bind(args.id);

        query.execute(pool).await?;
    }

    println!("{}", file_path);
    Ok(())
}
