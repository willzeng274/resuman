use std::{fs, path::PathBuf};

use chrono::{DateTime, Utc};
use clap::Parser;
use sqlx::SqlitePool;

use crate::config::Config;

#[derive(Parser, Debug)]
#[command(name = "create", about = "Create a new resume", aliases = ["new", "add"])]
pub struct CreateCommand {
    #[arg(short, long, required = true, help = "Company name")]
    pub company: String,

    #[arg(short, long, help = "Group name")]
    pub group: Option<String>,

    #[arg(short, long, help = "Template to use", conflicts_with = "file")]
    pub template: Option<String>,

    #[arg(short = 'f', long, help = "A tex file path to use as a template", conflicts_with = "template", aliases = ["tex", "copy"])]
    pub file: Option<PathBuf>,

    #[arg(short, long, help = "Position/role name", aliases = ["role", "job"])]
    pub position: Option<String>,

    // #[arg(short, long, help = "Date created")]
    // pub created_at: Option<DateTime<Utc>>,
    #[arg(long = "letter", help = "Applied with cover letter")]
    pub has_cover_letter: bool,

    // below is metadata
    #[arg(short, long, help = "Date applied", aliases = ["applied", "applied_at"])]
    pub applied_time: Option<DateTime<Utc>>,

    #[arg(short = 'd', long, help = "Length of job (Weeks)", aliases = ["duration"])]
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

pub async fn execute(cfg: Config, args: &CreateCommand, pool: &SqlitePool) {
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

    log::debug!("Folder path: {:?}", folder_path);
    log::debug!("File path: {:?}", file_path);

    // check if folder exists
    if folder_path.exists() {
        println!("Folder already exists: {:?}", folder_path);
        return;
    }

    // check if file exists
    if file_path.exists() {
        println!("File already exists: {:?}", file_path);
        return;
    }

    // check if template or copy file exists
    if template.is_empty() && copy_file.is_none() {
        println!("Template or copy file must be specified");
        return;
    }

    let contents: String;
    if copy_file.is_some() {
        let copy_file = copy_file.unwrap();
        let copy_content = fs::read_to_string(&copy_file);

        if copy_content.is_err() {
            println!("Copy file not found: {:?}", copy_file);
            return;
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
            println!("Template not found: {:?}", template_path);
            return;
        }

        contents = template_content.unwrap();
    }

    // create the folder
    fs::create_dir_all(&folder_path).unwrap();

    // create the resume file
    fs::File::create(&file_path).unwrap();

    fs::write(&file_path, &contents).unwrap();

    let resume_id = sqlx::query!(
        r#"
INSERT INTO resumes (company, "group", template, position, created_at, has_cover_letter)
VALUES (?1, ?2, ?3, ?4, ?5, ?6)
RETURNING id
        "#,
        args.company,
        group,
        template,
        position,
        now,
        args.has_cover_letter,
    )
    .fetch_one(pool)
    .await
    .unwrap()
    .id;

    sqlx::query!(
        r#"
INSERT INTO metadata (resume_id, applied_time, length, location, status, urls, notes)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        "#,
        resume_id,
        applied_time,
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
}
