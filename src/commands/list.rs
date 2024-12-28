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
        None => Err(anyhow::anyhow!("No subcommand provided")),
    }
}
