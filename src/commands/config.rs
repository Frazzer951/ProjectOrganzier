use crate::{config::Config, error::Error, utils::Result};
use clap::ArgMatches;

pub fn config(sub_matches: &ArgMatches, config: &mut Config) -> Result<()> {
    match sub_matches.subcommand() {
        Some(("set", sub_matches)) => {
            let key = sub_matches.get_one::<String>("key").cloned().unwrap();
            let value = sub_matches.get_one::<String>("value").cloned().unwrap();

            match key {
                key if key == "base" || key == "base-dir" => {
                    config.base_dir = Some(value.into());
                },
                key if key == "db" || key == "db-path" || key == "database-path" => {
                    config.database_path = value;
                },
                key if key == "template-dir" || key == "template" => {
                    config.template_dir = Some(value.into());
                },
                _ => {
                    return Err(Error::ConfigInvalidKey(key));
                },
            }

            config.save()?;
        },
        Some((command, _)) => {
            println!("Code has not yet been written for `{command}`");
        },
        _ => unreachable!(),
    }

    Ok(())
}
