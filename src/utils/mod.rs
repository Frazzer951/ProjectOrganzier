use crate::error::Error;
use dirs::config_dir;
use std::path::PathBuf;

pub type Result<T> = core::result::Result<T, Error>;

pub fn config_folder() -> PathBuf {
    let mut config_folder = config_dir().unwrap_or_default();
    config_folder.push("fpm");
    config_folder
}
