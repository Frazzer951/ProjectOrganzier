use crate::{config::Config, error::Error, utils::Result};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use fs_err as fs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub desc: Option<String>,
    pub template_dir: Option<PathBuf>,
    pub template_file: Option<PathBuf>,
    pub template_vars: Option<Vec<String>>,
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

        for template_str in templates {
            let template = match template_files.get(&template_str) {
                Some(template) => template,
                None => return Err(Error::TemplateNotFound(template_str)),
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
            // Walk the template dir and load all files and their contents
            let files = load_files(template_dir, variables)?;

            // Write the file to the project dir
            for (file, contents) in files {
                let file = path.join(file);
                fs::create_dir_all(file.parent().unwrap())?;
                fs::write(file, contents)?;
            }
        }

        if let Some(template_file) = &self.template_file {
            let file_contents = load_file(template_file, variables)?;
            let file = path.join(template_file);
            fs::create_dir_all(file.parent().unwrap())?;
            fs::write(file, file_contents)?;
        }

        if let Some(commands) = &self.commands {
            for command in commands {
                run_command(command, &path)?;
            }
        }

        Ok(())
    }
}

fn load_files(dir: &Path, variables: &HashMap<String, String>) -> Result<HashMap<PathBuf, String>> {
    let mut files = HashMap::new();

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            let sub_files = load_files(&path, variables)?;
            files.extend(sub_files);
        } else if path.is_file() {
            let contents = fs::read_to_string(&path)?;
            let contents = replace_variables(&contents, variables)?;
            files.insert(path, contents);
        }
    }

    // Remove directory prefix from keys
    let files = files
        .into_iter()
        .map(|(key, value)| (key.strip_prefix(dir).unwrap_or(&key).to_path_buf(), value))
        .collect();

    Ok(files)
}

fn load_file(path: &Path, variables: &HashMap<String, String>) -> Result<String> {
    let contents = fs::read_to_string(path)?;
    let contents = replace_variables(&contents, variables)?;
    Ok(contents)
}

fn replace_variables(contents: &str, variables: &HashMap<String, String>) -> Result<String> {
    let mut contents = contents.to_string();

    for (key, value) in variables {
        contents = contents.replace(&format!("{{{}}}", key), value);
    }

    Ok(contents)
}

fn run_command(command: &str, dir: &Path) -> Result<()> {
    let output = Command::new("sh").arg("-c").arg(command).current_dir(dir).output()?;

    if !output.status.success() {
        let stderr: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stderr);
        return Err(Error::CommandFailed(format!(
            "Command '{}' failed with error: {}",
            command, stderr
        )));
    }

    Ok(())
}
