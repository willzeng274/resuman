use crate::config::Config;
use anyhow::Result;
use clap::Parser;
use sqlx::SqlitePool;

#[derive(Parser, Debug)]
#[command(name = "find", about = "Find a resume", visible_aliases = ["search", "query"])]
pub struct FindCommand {
    // filters
    #[clap(short, long, about = "Filter by name")]
    name: Option<String>,
}

pub async fn execute(cfg: Config, args: &FindCommand, pool: &SqlitePool) -> Result<()> {
    log::debug!("Find arguments:\n{:#?}", args);

    Ok(())
}
