use crate::{config::Config, database, utils::Result};
use clap::ArgMatches;
use dialoguer::Confirm;

pub fn reset(sub_matches: &ArgMatches, config: &Config) -> Result<()> {
    let force = sub_matches.get_flag("force");

    if force
        || Confirm::new()
            .with_prompt("Are you sure you want to reset the entire database? This is irreversible")
            .interact()?
    {
        database::reset_database(config)?;
    }

    Ok(())
}
