mod commands;
mod error;
mod resume;
mod utils;

use std::env;
use std::fs;
use std::path::PathBuf;

use clap::{Arg, Command, Parser, Subcommand};
use commands::create;
use dotenv::dotenv;
use env_logger::Env;
use sqlx::SqlitePool;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Parser)]
#[command(name = NAME, version = VERSION, author = AUTHOR, about = DESCRIPTION)]
struct Cli {
    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Sets a custom config file path"
    )]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// A subcommand for creating something
    Create(create::CreateCommand),
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let env = Env::default();

    env_logger::init_from_env(env);

    let cli = Cli::parse();

    // switch to .clone if lifetime issues
    let config_path = cli.config;

    let config = utils::load_config(config_path).unwrap();

    log::debug!("{:?}", config);

    let root_dir = utils::resolve_path(config.config.root_dir).unwrap();
    log::debug!("{:?}", root_dir);

    let default_db_path = root_dir.join("resuman.db");
    let db_path = config.config.db_path.unwrap_or(default_db_path);

    fs::create_dir_all(&root_dir).unwrap();

    log::debug!("Db path: {:?}", db_path);

    let pool = SqlitePool::connect(db_path.to_str().unwrap()).await?;
    let file_path = format!("{}/test.txt", root_dir.display());
    utils::save_to_file(&file_path, "Hello, world!", Some(true)).unwrap();

    match cli.command {
        Some(Commands::Create(args)) => create::execute(&args),
        _ => eprintln!("Invalid subcommand or arguments"),
    }

    Ok(())
}
