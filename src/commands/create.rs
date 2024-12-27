use clap::{Arg, ArgAction, Command, Parser};

#[derive(Parser)]
#[command(name = "create", about = "Create a new resume")]
pub struct CreateCommand {
    #[arg(short, long, required = true, help = "Name of the resume")]
    name: String,

    #[arg(long, help = "Template to use")]
    template: Option<String>,

    #[arg(short, long, action = ArgAction::SetTrue, help = "Make the resume public")]
    public: bool,
}

pub fn create_command() -> Command {
    Command::new("create")
        .about("Create a new resume")
        .arg(Arg::new("name").short('n').long("name").required(true))
        .arg(Arg::new("template").long("template"))
        .arg(
            Arg::new("public")
                .short('p')
                .long("public")
                .action(ArgAction::SetTrue),
        )
}

pub fn execute(matches: &clap::ArgMatches) {
    let name = matches.get_one::<String>("name").unwrap();
    let template = matches.get_one::<String>("template");
    let public = matches.get_one::<bool>("public").unwrap_or(&false);

    println!(
        "Creating resume: {} with template: {:?} and public: {}",
        name, template, public
    );
}
