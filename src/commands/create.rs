use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use clap::Parser;
use serde_json::json;
use sqlx::SqlitePool;

use crate::config::Config;

#[derive(Parser, Debug)]
#[command(name = "create", about = "Create a new resume", visible_aliases = ["new", "add"])]
pub struct CreateCommand {
    #[arg(short, long, required = true, help = "Company name")]
    pub company: String,

    #[arg(short, long, help = "Group name")]
    pub group: Option<String>,

    #[arg(short, long, help = "Template to use", conflicts_with = "file")]
    pub template: Option<String>,

    #[arg(short = 'f', long, help = "A tex file path to use as a template", conflicts_with = "template", visible_aliases = ["tex", "copy"])]
    pub file: Option<PathBuf>,

    #[arg(short, long, help = "Position/role name", visible_aliases = ["role", "job"])]
    pub position: Option<String>,

    #[arg(short = 'a', long = "letter", help = "Applied with cover letter")]
    pub has_cover_letter: bool,

    // below is metadata
    #[arg(long, help = "Date applied", visible_aliases = ["applied", "applied_at"])]
    pub applied_time: Option<DateTime<Utc>>,

    #[arg(short = 'd', long, help = "Length of job (Weeks)", visible_aliases = ["duration"])]
    pub length: Option<u16>,

    #[arg(short, long, help = "Location of job")]
    pub location: Option<String>,

    #[arg(short, long, help = "Status of application")]
    pub status: Option<String>, // applying should be default
    // applying, applied, oa, interview, rejected, accepted, ghosted, etc
    #[arg(short, long, help = "URLs to job posting, company, etc")]
    pub urls: Option<String>,

    #[arg(short, long, help = "Other metadata")]
    pub notes: Option<String>,
}

pub async fn execute(cfg: Config, args: &CreateCommand, pool: &SqlitePool) -> Result<()> {
    // sqlite double query: insert into resumes AND metadata
    // use a CTE to insert into resumes first, then insert into metadata

    log::debug!("Creating with arguments:\n{:#?}", args);

    let group = args
        .group
        .clone()
        .unwrap_or(cfg.default_group.clone().unwrap_or("default".to_string()));
    let template = args.template.clone().unwrap_or(
        cfg.default_template
            .clone()
            .unwrap_or("default".to_string()),
    );
    let copy_file = args.file.clone(); // Option<PathBuf>
    let position = args
        .position
        .clone()
        .unwrap_or(cfg.default_position.clone().unwrap_or("swe".to_string()))
        .replace(" ", "-"); // replace spaces with dashes
    let now = Utc::now();

    let applied_time = args.applied_time;
    let length = args.length.unwrap_or(cfg.default_length.unwrap_or(16)); // default 4 months
    let location = args.location.clone(); // Option<String>
    let status = args
        .status
        .clone()
        .unwrap_or(cfg.default_status.unwrap_or("applying".to_string())); // default applying
    let urls = args.urls.clone(); // Option<String>
    let notes = args.notes.clone().unwrap_or("".to_string()); // default empty

    // log every property after applying defaults, in 1 big r string
    log::debug!(
        r#"
Group: {}
Template: {}
Copy file: {:?}
Position: {}
Created at: {}
Has cover letter: {}
Applied time: {:?}
Length: {}
Location: {:?}
Status: {}
URLs: {:?}
Notes: {}
        "#,
        group,
        template,
        copy_file,
        position,
        now,
        args.has_cover_letter,
        applied_time,
        length,
        location,
        status,
        urls,
        notes,
    );

    // try to create the resume before inserting into the database using Path
    let folder_pattern = cfg
        .folder_pattern
        .clone()
        .unwrap_or("{company}_{position}_{date}".to_string());
    let file_pattern = cfg
        .file_pattern
        .clone()
        .unwrap_or("resume_{company}_{position}_{date}.tex".to_string());
    let date_format = cfg.date_format.clone().unwrap_or("%Y-%m-%d".to_string());

    let root_dir = cfg.root_dir.clone().join(group.clone());
    log::debug!("Root directory: {:?}", root_dir);
    let company = args.company.clone().replace(" ", "-"); // replace spaces with dashes
    let date = now.format(&date_format).to_string();
    let folder = folder_pattern
        .replace("{company}", &company)
        .replace("{position}", &position)
        .replace("{date}", &date); // already formatted, it is the user's fault for not using dashes

    let file = file_pattern
        .replace("{company}", &company)
        .replace("{position}", &position)
        .replace("{date}", &date);

    let folder_path = root_dir.join(&folder);
    let file_path = folder_path.join(&file);
    let metadata_path = folder_path.join(
        cfg.metadata_name
            .clone()
            .unwrap_or("metadata.json".to_string())
            .replace("{company}", &company)
            .replace("{position}", &position)
            .replace("{date}", &date),
    );

    log::debug!("Folder path: {:?}", folder_path);
    log::debug!("File path: {:?}", file_path);
    log::debug!("Metadata path: {:?}", metadata_path);

    // check if folder exists
    if folder_path.exists() {
        return Err(anyhow!("Folder already exists: {:?}", folder_path));
    }

    // check if file exists
    // should be unreachable
    if file_path.exists() {
        return Err(anyhow!("File already exists: {:?}", file_path));
    }

    // check if template or copy file exists
    // should be unreachable
    if template.is_empty() && copy_file.is_none() {
        return Err(anyhow!("Template or copy file must be specified"));
    }

    // should be unreachable
    if metadata_path.exists() {
        log::warn!("Metadata file already exists: {:?}", metadata_path);
    }

    let contents: String;
    if copy_file.is_some() {
        let copy_file = copy_file.clone().unwrap();
        let copy_content = fs::read_to_string(&copy_file);

        if copy_content.is_err() {
            return Err(anyhow!("Copy file not found: {:?}", copy_file));
        }

        contents = copy_content.unwrap();
    } else {
        let template_path = cfg.root_dir.clone().join(
            cfg.template_dir
                .clone()
                .unwrap_or(PathBuf::from("templates"))
                .join(&template),
        );
        let template_content = fs::read_to_string(&template_path.with_extension("tex"));

        if template_content.is_err() {
            return Err(anyhow!("Template not found: {:?}", template_path));
        }

        contents = template_content.unwrap();
    }

    // create the folder
    fs::create_dir_all(&folder_path).unwrap();

    // create the resume file
    fs::write(&file_path, &contents).unwrap();

    // create the metadata file
    // serialize datetimes to number timestamps
    let metadata = json!({
        "company": args.company,
        "group": group,
        "template": template,
        "position": position,
        "created_at": now.timestamp(),
        "has_cover_letter": args.has_cover_letter,
        "applied_time": applied_time.map(|t| t.timestamp()),
        "length": length,
        "copied_from": copy_file.clone().map(|f| f.display().to_string()),
        "location": location,
        "status": status,
        "urls": urls,
        "notes": notes,
    });

    fs::write(
        &metadata_path,
        serde_json::to_string_pretty(&metadata).unwrap(),
    )
    .unwrap();

    if args.has_cover_letter {
        let cover_letter_path = folder_path.join(
            cfg.cover_letter_name
                .clone()
                .unwrap_or("cover_letter.txt".to_string())
                .replace("{company}", &company)
                .replace("{position}", &position)
                .replace("{date}", &date),
        );
        fs::write(&cover_letter_path, "").unwrap();
    }

    let stored_path = file_path.display().to_string();
    let resume_id = sqlx::query!(
        r#"
INSERT INTO resumes (company, "group", template, position, created_at, has_cover_letter, file_path)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
RETURNING id
        "#,
        args.company,
        group,
        template, // does not support copy_file
        position,
        now,
        args.has_cover_letter,
        stored_path,
    )
    .fetch_one(pool)
    .await
    .unwrap()
    .id;

    let copied_from = copy_file.map(|f| f.display().to_string());
    let metadata_file_path = metadata_path.display().to_string();
    sqlx::query!(
        r#"
INSERT INTO metadata (resume_id, applied_time, copied_from, metadata_file_path, length, location, status, urls, notes)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        "#,
        resume_id,
        applied_time,
        copied_from,
        metadata_file_path,
        length,
        location,
        status,
        urls,
        notes,
    )
    .execute(pool)
    .await
    .unwrap();

    // Print the path to the file with no message
    println!("{}", file_path.display());
    Ok(())
}
