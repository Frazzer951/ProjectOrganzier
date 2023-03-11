use crate::error::Error;
use dirs::config_dir;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use std::time::Duration;

pub type Result<T> = core::result::Result<T, Error>;

pub fn config_folder() -> PathBuf {
    let mut config_folder = config_dir().unwrap_or_default();
    config_folder.push("fpm");
    config_folder
}

pub fn create_spinner(msg: &str) -> Result<ProgressBar> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_style(
        ProgressStyle::with_template("[{elapsed}] {spinner:.blue} - {msg}")?
            .tick_strings(&["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"]),
    );

    pb.set_message(msg.to_owned());
    Ok(pb)
}
