use std::path::PathBuf;

use clap::{Parser, Subcommand};

pub mod create;
pub mod list;
// pub mod remove;
// pub mod update;
// pub mod find;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[derive(Parser)]
#[command(name = NAME, version = VERSION, author = AUTHOR, about = DESCRIPTION, arg_required_else_help = true)]
pub struct Cli {
    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "Sets a custom config file path"
    )]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Parser)]
pub struct InitCommand;

#[derive(Subcommand)]
pub enum Commands {
    /// A subcommand for creating something
    Create(create::CreateCommand),
    List(list::ListCommand),
    Init(InitCommand)
}
