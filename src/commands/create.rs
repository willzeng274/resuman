use clap::{Arg, ArgAction, Command};

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
