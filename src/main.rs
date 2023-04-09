use anyhow::Result;
use clap::Parser;
use log::debug;
use console::style;

mod cli;
mod config;

use crate::cli::*;


fn main() -> Result<()> {
    let arguments = Arguments::parse();

    let log_level = match arguments.debug {
        false => log::LevelFilter::Info,
        true => log::LevelFilter::Debug
    };

    env_logger::builder()
        .parse_default_env()
        .filter_level(log_level);

    if !arguments.disable_banner {
        println!(
            "{}    {} - v{}",
            style(BANNER).green(),
            style(AUTHOR).red(),
            style(VERSION_NUMBER).blue()
        );
    }

    debug!("Finished initialising, starting main workflow...");

    // Subcommands 
    match &arguments.commands {
        ArgumentCommands::Run => {
            todo!("Run sub-command")
        },
        ArgumentCommands::Test => {
            todo!("Run tests")
        }
    }
    
}
