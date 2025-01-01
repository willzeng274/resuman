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

// if the user creates a resume without deleting it properly,
// then the resume will be orphaned in the db...
// but if the user has created another resume with the same file path
// there will be a conflict? but this case can be handled by
// keeping the latest resume and deleting the older ones with the same file path
#[derive(Parser)]
#[command(name = "clean", about = "Clean up resuman", visible_aliases = ["clear"], long_about = "Clean up resuman sqlite database by checking for missing paths")]
pub struct CleanCommand;

#[derive(Subcommand)]
pub enum Commands {
    /// A subcommand for creating something
    Create(create::CreateCommand),
    List(list::ListCommand),
    Init(InitCommand),
    Update(update::UpdateCommand),
    Delete(delete::DeleteCommand),
    Find(find::FindCommand),
    Clean(CleanCommand),
}
