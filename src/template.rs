use crate::config::Config;
use crate::error::Error;
use crate::project::Project;
use crate::utils::{config_folder, Result};
use fs_err as fs;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub desc: Option<String>,
    pub template_dir: Option<PathBuf>,
    pub template_file: Option<PathBuf>,
    pub template_vars: Option<Vec<String>>,
    pub sub_templates: Option<Vec<String>>,
}

impl Template {
    pub fn load_templates(config: &Config) -> Result<Vec<Template>> {
        if config.template_dir.is_none() {
            return Err(Error::ConfigMissingValue("template_dir".to_owned()));
        }

        let templates = vec![];

        Ok(templates)
    }
}
