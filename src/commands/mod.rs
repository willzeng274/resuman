use std::path::PathBuf;

use clap::{Parser, Subcommand};

pub mod create;
pub mod delete;
pub mod find;
pub mod list;
pub mod update;

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

    #[arg(long, hide = true)]
    pub markdown_help: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Parser)]
#[command(name = "init", about = "Initialize resuman")]
pub struct InitCommand;

#[derive(Subcommand)]
pub enum Commands {
    /// A subcommand for creating something
    Create(create::CreateCommand),
    List(list::ListCommand),
    Init(InitCommand),
    Update(update::UpdateCommand),
    Delete(delete::DeleteCommand),
}
