mod commands;
mod error;
mod resume;
mod utils;

use std::fs;
use std::path::PathBuf;

use clap::{Arg, Command, Parser, Subcommand};
use commands::create;
use dotenv::dotenv;
use env_logger::Env;

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

fn main() {
    dotenv().ok();

    let env = Env::default();

    env_logger::init_from_env(env);

    let matches = Command::new(NAME)
        .version(VERSION)
        .author(AUTHOR)
        .about(DESCRIPTION)
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file path")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .subcommand(create::create_command())
        .get_matches();

    // clone to avoid lifetime issues
    let config_path = matches.get_one::<PathBuf>("config").cloned();

    let config = utils::load_config(config_path).unwrap();
    log::debug!("{:?}", config);

    let root_dir = utils::resolve_path(config.config.root_dir).unwrap();
    log::debug!("{:?}", root_dir);

    fs::create_dir_all(&root_dir).unwrap();

    let file_path = format!("{}/test.txt", root_dir.display());
    utils::save_to_file(&file_path, "Hello, world!", Some(true)).unwrap();

    match matches.subcommand() {
        Some(("create", sub_matches)) => create::execute(sub_matches),
        _ => eprintln!("Invalid subcommand or arguments"),
    }
}
