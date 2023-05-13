mod project;

mod cli;
mod commands;
mod config;
mod database;
mod error;
mod template;

mod utils;

use eyre::Result;

fn main() -> Result<()> {
    cli::parse()?;

    Ok(())
}
