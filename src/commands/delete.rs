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

pub async fn execute(cfg: Config, args: &DeleteCommand, pool: &SqlitePool) -> Result<()> {
    log::debug!("Deleting with arguments:\n{:#?}", args);

    Ok(())
}
