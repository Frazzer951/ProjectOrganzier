use crate::error::Error;
use crate::project::Project;
use crate::utils::{config_folder, Result};
use fs_err as fs;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;

pub const CONFIG_FILE: &str = "ProjectOrganizer.toml";
pub const DATABASE_FILE: &str = "projectDB.db";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub database_path: String,
    pub base_dir: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        let cf = config_folder();

        let mut db_path = cf;
        db_path.push(DATABASE_FILE);

        Self {
            database_path: db_path.to_str().unwrap_or_default().to_owned(),
            base_dir: None,
        }
    }

    pub fn load() -> Result<Self> {
        let cf = config_folder();
        let mut config_path = cf;
        config_path.push(CONFIG_FILE);

        let content = match fs::read_to_string(config_path) {
            Ok(c) => c,
            Err(_) => return Err(Error::ConfigNotFound),
        };

        match toml::from_str::<Config>(&content) {
            Ok(c) => Ok(c),
            Err(e) => Err(Error::TomlDes(e)),
        }
    }

    pub fn save(&self) -> Result<()> {
        let cf = config_folder();
        let mut config_path = cf.clone();
        config_path.push(CONFIG_FILE);

        fs::create_dir_all(cf)?;

        let mut file = match fs::File::create(config_path) {
            Ok(f) => f,
            Err(e) => return Err(Error::IO(e)),
        };

        let serialized = match toml::to_string(self) {
            Ok(s) => s,
            Err(e) => return Err(Error::TomlSer(e)),
        };

        match file.write(serialized.as_bytes()) {
            Ok(_) => {},
            Err(e) => return Err(Error::IO(e)),
        };

        Ok(())
    }

    pub fn gen_project_folder(&self, project: &Project) -> Result<PathBuf> {
        let mut path = PathBuf::new();

        if let Some(base_dir) = &self.base_dir {
            path.push(base_dir);
        } else {
            return Err(Error::ConfigMissingValue("base_dir".to_owned()));
        }

        if let Some(cat) = &project.category {
            path.push(cat);
        }
        if let Some(lang) = &project.language {
            path.push(lang);
        }
        if let Some(name) = &project.name {
            path.push(name);
        }

        Ok(path)
    }
}
