// default only removes the file in the sql db, not the actual file
// --file flag should be used to remove the actual file
// there's no reason to remove the file but not from the db...

use crate::config::Config;
use anyhow::Result;
use clap::Parser;
use sqlx::SqlitePool;

#[derive(Parser, Debug)]
#[command(name = "delete", about = "Delete a resume", visible_aliases = ["remove", "rm"])]
pub struct DeleteCommand {
    #[arg(index = 1, help = "ID of the resume to delete")]
    pub id: i32,

    #[arg(short, long, help = "Remove the actual file")]
    pub file: bool,
}

pub async fn execute(_cfg: Config, args: &DeleteCommand, pool: &SqlitePool) -> Result<()> {
    log::debug!("Deleting with arguments:\n{:#?}", args);

    if args.file {
        // run sql queries to get file path from db
        // run fs commands to delete file
        let path = sqlx::query!("SELECT file_path FROM resumes WHERE id = ?", args.id)
            .fetch_one(pool)
            .await?
            .file_path;

        // get the entire directory of the file and remove that
        std::fs::remove_dir_all(path)?;
    }

    // run sql queries to delete resume from db
    // run sql queries to delete resume from db
    sqlx::query!("DELETE FROM resumes WHERE id = ?", args.id)
        .execute(pool)
        .await?;

    Ok(())
}
