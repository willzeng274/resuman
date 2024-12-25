mod commands;
mod error;
mod resume;
mod utils;

use clap::Command;
use commands::create;

fn main() {
    let matches = Command::new("resume-manager")
        .version("1.0")
        .author("Your Name <you@example.com>")
        .about("CLI tool for managing resumes")
        .subcommand(create::create_command())
        // .subcommand(remove::remove_command())
        // .subcommand(update::update_command())
        // .subcommand(find::find_command())
        .get_matches();

    match matches.subcommand() {
        Some(("create", sub_matches)) => create::execute(sub_matches),
        // Some(("remove", sub_matches)) => remove::execute(sub_matches),
        // Some(("update", sub_matches)) => update::execute(sub_matches),
        // Some(("find", sub_matches)) => find::execute(sub_matches),
        _ => eprintln!("Invalid subcommand or arguments"),
    }
}
