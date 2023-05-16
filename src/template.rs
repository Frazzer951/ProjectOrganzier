use crate::{config::Config, error::Error, utils::Result};
use fs_err as fs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
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
    pub fn load_templates(config: &Config) -> Result<HashMap<String, Template>> {
        let template_dir = match config.template_dir {
            Some(ref dir) => dir,
            None => return Err(Error::ConfigMissingValue("template_dir".to_owned())),
        };

        let mut templates = HashMap::new();

        for entry in fs::read_dir(template_dir)? {
            let path = entry?.path();
            if path.is_file()
                && (path.extension().unwrap_or_default() == "yaml" || path.extension().unwrap_or_default() == "yml")
            {
                let contents = fs::read_to_string(&path)?;
                let template: Template = serde_yaml::from_str(&contents)?;

                templates.insert(template.name.clone(), template);
            }
        }

        Ok(templates)
    }

    pub fn build_templates(path: PathBuf, templates: Vec<String>, template_files: &HashMap<String, Template>) -> Result<()> {
        let mut variables: HashMap<String, String> = HashMap::new();

        Ok(())
    }

    pub fn build(self, path: PathBuf, variables: &mut HashMap<String, String>) -> Result<()> {
        Ok(())
    }
}
