use crate::{config::Config, template::Template};
use fs_err as fs;
use std::{collections::HashMap, path::PathBuf};
use turbosql::Turbosql;

#[derive(Turbosql, Default, Debug, PartialEq, Eq, Clone)]
pub struct Project {
    pub rowid: Option<i64>,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub directory: Option<PathBuf>,
    pub tags: Option<Vec<String>>,
    pub language: Option<String>,
    pub category: Option<String>,
}

impl Project {
    pub fn new(
        name: Option<String>,
        desc: Option<String>,
        tags: Vec<String>,
        language: Option<String>,
        category: Option<String>,
    ) -> Self {
        Project {
            rowid: None,
            name,
            desc,
            directory: None,
            tags: Some(tags),
            language,
            category,
        }
    }

    pub fn build(&mut self, dir: Option<PathBuf>, config: &Config, templates: Vec<String>) -> crate::utils::Result<()> {
        let dir = if let Some(dir) = dir {
            self.directory = Some(dir.clone());
            dir
        } else {
            let dir = config.gen_project_folder(self)?;
            self.directory = Some(dir.clone());
            dir
        };

        let template_files = if templates.is_empty() {
            HashMap::new()
        } else {
            Template::load_templates(config)?
        };

        fs::create_dir_all(&dir)?;

        Template::build_templates(dir, templates, &template_files)?;

        Ok(())
    }
}
