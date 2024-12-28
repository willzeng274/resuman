use std::{fs::read_dir, path::PathBuf};

use clap::Parser;
use sqlx::SqlitePool;

use crate::config::Config;

#[derive(Parser)]
#[command(name = "list", about = "Command related to resume groups")]
pub struct ListCommand {
    #[command(subcommand)]
    pub command: Option<ListCommands>,
}

#[derive(Parser)]
pub enum ListCommands {
    /// a subcommand for listing all groups
    Group(GroupCommand),
    Template(TemplateCommand),
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
    #[arg(short, long, help = "Verbose output")]
    pub verbose: bool,

    #[arg(short, long, help = "Use fs instead of SQL db")]
    pub fs: bool,
}

pub async fn execute(cfg: Config, args: &ListCommand, pool: &SqlitePool) {
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
                return;
            }

            // sql query to find all groups in the database with "resume" table, "group" column
            let records = sqlx::query!("SELECT DISTINCT \"group\" FROM resumes")
                .fetch_all(pool)
                .await
                .unwrap();
            records.iter().for_each(|r| println!("{}", r.group));
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
                return;
            }

            // sql query to find all templates in the database with "resume" table, "template" column
            let records = sqlx::query!("SELECT DISTINCT template FROM resumes")
                .fetch_all(pool)
                .await
                .unwrap();
            records.iter().for_each(|r| println!("{}", r.template));
        }
        None => {
            println!("No subcommand provided");
        }
    }
}
