use std::{fs::read_dir, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use sqlx::SqlitePool;
use walkdir::WalkDir;

use crate::config::Config;

#[derive(Parser)]
#[command(
    name = "list",
    about = "Command related to resume groups",
    arg_required_else_help = true
)]
pub struct ListCommand {
    #[command(subcommand)]
    pub command: Option<ListCommands>,
}

#[derive(Parser)]
pub enum ListCommands {
    /// a subcommand for listing all groups
    Group(GroupCommand),
    Template(TemplateCommand),
    Flatten(FlattenCommand),
    All(AllCommand),
}

#[derive(Parser)]
#[command(name = "group", about = "List all groups")]
pub struct GroupCommand {
    #[arg(short, long, help = "Verbose output", requires("fs"))]
    pub verbose: bool,

    #[arg(short, long, help = "Use fs instead of SQL db")]
    pub fs: bool,
}

#[derive(Parser)]
#[command(name = "template", about = "List all templates")]
pub struct TemplateCommand {
    #[arg(short, long, help = "Verbose output", requires("fs"))]
    pub verbose: bool,

    #[arg(short, long, help = "Use fs instead of SQL db")]
    pub fs: bool,
}

#[derive(Parser)]
#[command(name = "flatten", about = "List all resumes")]
pub struct FlattenCommand {
    #[arg(short, long, help = "Verbose output", requires("fs"))]
    pub verbose: bool,

    #[arg(short, long, help = "Use fs instead of SQL db")]
    pub fs: bool,

    // ignore flag for folders, can be specified multiple times
    #[arg(
        short,
        long,
        help = "Ignore flag for folders/directories, exact match",
        requires("fs")
    )]
    pub ignore: Vec<String>,
}

#[derive(Parser)]
#[command(name = "all", about = "List all resumes in columns")]
pub struct AllCommand {
    // everything
    #[arg(short, long, help = "Verbose output")]
    pub verbose: bool,

    // an argument for every column in the resume table
    #[arg(short, long, help = "ID of the resume")]
    pub id: bool,

    #[arg(short, long, help = "Group of the resume")]
    pub group: bool,

    #[arg(short, long, help = "Template of the resume")]
    pub template: bool,

    #[arg(short, long, help = "Company of the resume")]
    pub company: bool,

    #[arg(long = "letter", help = "Cover letter status of the resume")]
    pub has_cover_letter: bool,

    #[arg(short, long, help = "File path of the resume")]
    pub file_path: bool,

    #[arg(long, help = "Date created of the resume")]
    pub created_at: bool,

    #[arg(short, long, help = "Applied time of the resume")]
    pub applied_time: bool,

    #[arg(long, help = "Copied from of the resume")]
    pub copied_from: bool,

    #[arg(short, long, help = "Metadata file path of the resume")]
    pub metadata_file_path: bool,

    #[arg(long, help = "Length of the job")]
    pub length: bool,

    #[arg(short, long, help = "Location of the job")]
    pub location: bool,

    #[arg(short, long, help = "Status of the application")]
    pub status: bool,

    #[arg(short, long, help = "URLs of the resume")]
    pub urls: bool,

    #[arg(short, long, help = "Notes of the resume")]
    pub notes: bool,
}

pub async fn execute(cfg: Config, args: &ListCommand, pool: &SqlitePool) -> Result<()> {
    match &args.command {
        Some(ListCommands::Group(args)) => {
            if args.fs {
                // find all directories in the root directory
                let entries = read_dir(cfg.root_dir.clone()).unwrap();
                entries
                    .filter_map(|entry| {
                        let entry = entry.unwrap();
                        let path = entry.path();
                        if path.is_dir() {
                            Some(path)
                        } else {
                            None
                        }
                    })
                    .for_each(|path| {
                        if args.verbose {
                            println!("{}", path.display())
                        } else {
                            // don't output full path
                            println!("{}", path.file_name().unwrap().to_str().unwrap())
                        }
                    });
                return Ok(());
            }

            // sql query to find all groups in the database with "resume" table, "group" column
            let records = sqlx::query!("SELECT DISTINCT \"group\" FROM resumes")
                .fetch_all(pool)
                .await
                .unwrap();
            records.iter().for_each(|r| println!("{}", r.group));
            Ok(())
        }
        Some(ListCommands::Template(args)) => {
            // find all files in the templates directory

            if args.fs {
                let template_path = cfg.root_dir.clone().join(
                    cfg.template_dir
                        .clone()
                        .unwrap_or(PathBuf::from("templates")),
                );
                log::debug!("Template path: {:?}", template_path);
                let entries = read_dir(template_path).unwrap();
                entries
                    .filter_map(|entry| {
                        let entry = entry.unwrap();
                        let path = entry.path();
                        if path.is_file() {
                            Some(path)
                        } else {
                            None
                        }
                    })
                    .for_each(|path| {
                        if args.verbose {
                            println!("{}", path.display())
                        } else {
                            // don't output full path or the extension
                            println!("{}", path.file_stem().unwrap().to_str().unwrap())
                        }
                    });
                return Ok(());
            }

            // sql query to find all templates in the database with "resume" table, "template" column
            let records = sqlx::query!("SELECT DISTINCT template FROM resumes")
                .fetch_all(pool)
                .await
                .unwrap();
            records.iter().for_each(|r| println!("{}", r.template));
            Ok(())
        }
        Some(ListCommands::Flatten(args)) => {
            // find all .tex files in the root directory
            // some inconsistencies here between sql and fs, but it's fine for now

            if args.fs {
                WalkDir::new(cfg.root_dir.clone())
                    .into_iter()
                    .filter_entry(|entry| {
                        let path = entry.path();
                        // Skip directories in the ignore list
                        !args.ignore.iter().any(|ignore| path.ends_with(ignore))
                    })
                    .filter_map(|entry| {
                        let entry = entry.ok()?;
                        let path = entry.path().to_path_buf();
                        if path.is_file() && path.extension().unwrap_or_default() == "tex" {
                            Some(path)
                        } else {
                            None
                        }
                    })
                    .for_each(|path| {
                        if args.verbose {
                            println!("{}", path.display())
                        } else {
                            // don't output full path or the extension
                            println!("{}", path.file_stem().unwrap().to_str().unwrap())
                        }
                    });
                return Ok(());
            }

            // sql query to find all files in the database with "resume" table, "file_path" column
            // this is the file path of the resume
            // unfortunately this will always be verbose (full path)
            let records = sqlx::query!("SELECT file_path FROM resumes")
                .fetch_all(pool)
                .await
                .unwrap();
            records.iter().for_each(|r| println!("{}", r.file_path));
            Ok(())
        }
        Some(ListCommands::All(args)) => {
            // sql query to find all resumes in the database + fields
            // display fields based on args
            let mut fields = Vec::new();

            if args.id {
                fields.push("id".to_string());
            }
            if args.group {
                fields.push("\"group\"".to_string());
            }
            if args.template {
                fields.push("template".to_string());
            }
            if args.company {
                fields.push("company".to_string());
            }
            if args.has_cover_letter {
                fields.push("has_cover_letter".to_string());
            }
            if args.file_path {
                fields.push("file_path".to_string());
            }
            if args.created_at {
                fields.push("created_at".to_string());
            }
            if args.applied_time {
                fields.push("applied_time".to_string());
            }
            if args.copied_from {
                fields.push("copied_from".to_string());
            }
            if args.metadata_file_path {
                fields.push("metadata_file_path".to_string());
            }
            if args.length {
                fields.push("length".to_string());
            }
            if args.location {
                fields.push("location".to_string());
            }
            if args.status {
                fields.push("status".to_string());
            }
            if args.urls {
                fields.push("urls".to_string());
            }
            if args.notes {
                fields.push("notes".to_string());
            }

            let records = sqlx::query!(
                "SELECT * FROM resumes JOIN metadata ON resumes.id = metadata.resume_id"
            )
            .fetch_all(pool)
            .await
            .unwrap();

            // display a row of fields first separated by tabs
            // then display each record separated by tabs
            if args.verbose {
                println!("id\tgroup\ttemplate\tcompany\thas_cover_letter\tfile_path\tcreated_at\tapplied_time\tcopied_from\tmetadata_file_path\tlength\tlocation\tstatus\turls\tnotes");
            } else {
                let header = fields.join("\t");
                println!("{}", header);
            }

            for record in records {
                let mut row = Vec::new();
                if args.verbose {
                    row.push(record.id.to_string());
                    row.push(record.group.clone());
                    row.push(record.template.clone());
                    row.push(record.company.clone());
                    row.push(record.has_cover_letter.to_string());
                    row.push(record.file_path.clone());
                    row.push(record.created_at.to_string());
                    // for optional fields display null if None
                    row.push(
                        record
                            .applied_time
                            .map(|dt| dt.to_string())
                            .unwrap_or_else(|| "null".to_string()),
                    );
                    row.push(record.copied_from.clone().unwrap_or("null".to_string()));
                    row.push(record.metadata_file_path.clone());
                    row.push(record.length.to_string());
                    row.push(record.location.clone().unwrap_or("null".to_string()));
                    row.push(record.status.clone());
                    row.push(record.urls.clone().unwrap_or("null".to_string()));
                    row.push(record.notes.clone());
                    println!("{}", row.join("\t"));
                    continue;
                }
                if args.id {
                    row.push(record.id.to_string());
                }
                if args.group {
                    row.push(record.group.clone());
                }
                if args.template {
                    row.push(record.template.clone());
                }
                if args.company {
                    row.push(record.company.clone());
                }
                if args.has_cover_letter {
                    row.push(record.has_cover_letter.to_string());
                }
                if args.file_path {
                    row.push(record.file_path.clone());
                }
                if args.created_at {
                    row.push(record.created_at.to_string());
                }
                if args.applied_time {
                    row.push(
                        record
                            .applied_time
                            .map(|dt| dt.to_string())
                            .unwrap_or_else(|| "null".to_string()),
                    );
                }
                if args.copied_from {
                    row.push(record.copied_from.clone().unwrap_or("null".to_string()));
                }
                if args.metadata_file_path {
                    row.push(record.metadata_file_path.clone());
                }
                if args.length {
                    row.push(record.length.to_string());
                }
                if args.location {
                    row.push(record.location.clone().unwrap_or("null".to_string()));
                }
                if args.status {
                    row.push(record.status.clone());
                }
                if args.urls {
                    row.push(record.urls.clone().unwrap_or("null".to_string()));
                }
                if args.notes {
                    row.push(record.notes.clone());
                }
                println!("{}", row.join("\t"));
            }

            Ok(())
        }
        None => Err(anyhow::anyhow!("No subcommand provided")),
    }
}
