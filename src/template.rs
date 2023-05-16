use crate::{config::Config, error::Error, utils::Result};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use fs_err as fs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::todo;

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub desc: Option<String>,
    pub template_dir: Option<PathBuf>,
    pub template_file: Option<PathBuf>,
    pub template_vars: Option<Vec<String>>,
    pub sub_templates: Option<Vec<String>>,
    pub commands: Option<Vec<String>>,
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

        for template in templates {
            let template = match template_files.get(&template) {
                Some(template) => template,
                None => return Err(Error::TemplateNotFound(template)),
            };

            template.build(path.clone(), &mut variables)?;
        }

        Ok(())
    }

    pub fn build(&self, path: PathBuf, variables: &mut HashMap<String, String>) -> Result<()> {
        if let Some(vars) = &self.template_vars {
            for var in vars {
                if !variables.contains_key(var) {
                    let var_value = Some(
                        Input::with_theme(&ColorfulTheme::default())
                            .with_prompt(format!("Enter the value for {var}"))
                            .interact_text()?,
                    );
                    variables.insert(var.to_string(), var_value.unwrap_or_default());
                }
            }
        }

        if let Some(template_dir) = &self.template_dir {
            todo!("build template dir")
            //Template::build_template_dir(path.clone(), template_dir, &mut variables)?;
        }

        if let Some(template_file) = &self.template_file {
            todo!("build template file")
            //Template::build_template_file(path.clone(), template_file, &mut variables)?;
        }

        Ok(())
    }
}
