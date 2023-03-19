use crate::{commands, config::Config, error::Error, utils::Result};
use clap::{command, value_parser, Arg, ArgAction, Command};
use std::path::PathBuf;

fn cli() -> Command {
    command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands(vec![
            subcommand_new(),
            subcommand_add(),
            subcommand_list(),
            subcommand_reset(),
        ])
}

fn subcommand_new() -> Command {
    Command::new("new").about("Create a New Project").args(&[
        Arg::new("name").short('n').long("name").help("Project Name"),
        Arg::new("desc").long("desc").help("Description of the project"),
        Arg::new("directory")
            .short('d')
            .long("directory")
            .help("The directory to place the project in. If nothing is provided a directory will be generated")
            .value_parser(value_parser!(PathBuf)),
        Arg::new("tags")
            .long("tag")
            .num_args(1..)
            .action(ArgAction::Append)
            .help("Tags for the project"),
        Arg::new("language")
            .short('l')
            .long("language")
            .help("Primary programming language used"),
        Arg::new("category")
            .short('c')
            .long("category")
            .help("Used to keep similar project types together. I.E. `work`, `thirdparty`, etc"),
        Arg::new("interactive")
            .short('i')
            .long("interactive")
            .action(ArgAction::SetTrue),
    ])
}

fn subcommand_add() -> Command {
    Command::new("add").about("Add an Existing Project").args(&[
        Arg::new("name").short('n').long("name").help("Project Name"),
        Arg::new("desc").long("desc").help("Description of the project"),
        Arg::new("directory")
            .short('d')
            .long("directory")
            .help("The directory to place the project in. If nothing is provided a directory will be generated")
            .value_parser(value_parser!(PathBuf)),
        Arg::new("tags")
            .long("tag")
            .num_args(1..)
            .action(ArgAction::Append)
            .help("Tags for the project"),
        Arg::new("language")
            .short('l')
            .long("language")
            .help("Primary programming language used"),
        Arg::new("category")
            .short('c')
            .long("category")
            .help("Used to keep similar project types together. I.E. `work`, `thirdparty`, etc"),
        Arg::new("interactive")
            .short('i')
            .long("interactive")
            .action(ArgAction::SetTrue),
    ])
}

fn subcommand_list() -> Command {
    Command::new("list").about("List the projects in the database")
}

fn subcommand_reset() -> Command {
    Command::new("reset")
        .about("Reset the project database to be empty")
        .args(&[Arg::new("force")
            .short('f')
            .long("force")
            .help("Bypass conformation prompt and reset the database")
            .action(ArgAction::SetTrue)])
}

pub fn parse() -> Result<()> {
    let matches = cli().get_matches();

    let config = match Config::load() {
        Err(Error::ConfigNotFound) => {
            println!("Config not found. Creating a new one.");
            let config = Config::new();
            config.save()?;
            Ok(config)
        },
        result => result,
    }?;

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            commands::new::new(sub_matches, &config)?;
        },
        Some(("add", sub_matches)) => {
            commands::add::add(sub_matches, &config)?;
        },
        Some(("list", _)) => {
            commands::list::list(&config)?;
        },
        Some(("reset", sub_matches)) => {
            commands::reset::reset(sub_matches, &config)?;
        },
        Some((command, _)) => {
            println!("Code has not yet been written from `{command}`");
        },
        _ => unreachable!(),
    }

    Ok(())
}
