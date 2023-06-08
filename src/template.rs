use crate::{config::Config, error::Error, utils::Result};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use fs_err as fs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
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

    pub fn build_templates(
        path: PathBuf,
        templates: Vec<String>,
        template_files: &HashMap<String, Template>,
        config: &Config,
    ) -> Result<()> {
        let mut variables: HashMap<String, String> = HashMap::new();

        for template_str in templates {
            let template = match template_files.get(&template_str) {
                Some(template) => template,
                None => return Err(Error::TemplateNotFound(template_str)),
            };

            template.build(path.clone(), &mut variables, config)?;
        }

        Ok(())
    }

    pub fn build(&self, path: PathBuf, variables: &mut HashMap<String, String>, config: &Config) -> Result<()> {
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
            let template_dir = config.template_dir.as_ref().unwrap().join(template_dir);
            // Walk the template dir and load all files and their contents
            let files = load_files(&template_dir, variables)?;

            // Write the file to the project dir
            for (file, contents) in files {
                let file = path.join(file);
                fs::create_dir_all(file.parent().unwrap())?;
                fs::write(file, contents)?;
            }
        }

        if let Some(template_file) = &self.template_file {
            let template_file = config.template_dir.as_ref().unwrap().join(template_file);
            let file_contents = load_file(&template_file, variables)?;
            let file = path.join(template_file.file_name().unwrap());
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
        .map(|(key, value)| (key.strip_prefix(dir.parent().unwrap()).unwrap_or(&key).to_path_buf(), value))
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
        contents = contents.replace(&format!("${{{}}}", key), value);
    }

    Ok(contents)
}

fn run_command(command: &str, dir: &Path) -> Result<()> {
    // split the command from the arguments
    let mut split_command = command.split_whitespace();
    let prog = split_command.next().unwrap();
    let args = split_command.collect::<Vec<&str>>();

    let mut cmd = Command::new(prog);
    cmd.current_dir(dir);

    for arg in args {
        cmd.arg(arg);
    }
    let output = match cmd.output() {
        Ok(output) => output,
        Err(e) => {
            return Err(Error::CommandFailed(format!(
                "Command '{}' failed with error: {}",
                command, e
            )))
        },
    };

    if !output.status.success() {
        let stderr: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stderr);
        return Err(Error::CommandFailed(format!(
            "Command '{}' failed with error: {}",
            command, stderr
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use anyhow::Result;

    #[test]
    fn test_load_templates() {
        let config = Config {
            template_dir: Some(PathBuf::from("tests/templates")),
            ..Default::default()
        };

        let expected_template = Template {
            name: "test".to_owned(),
            desc: Some("A Test Tempalte".to_owned()),
            template_dir: Some(PathBuf::from("template_dir")),
            template_file: Some(PathBuf::from("file_3.txt")),
            template_vars: Some(vec!["number".to_owned(), "number2".to_owned()]),
            commands: Some(vec!["git init".to_owned()]),
        };

        let tempaltes = Template::load_templates(&config).unwrap();
        let template = tempaltes.get("test").unwrap();

        assert_eq!(*template, expected_template);
    }

    #[test]
    fn test_load_file() {
        let mut variables = HashMap::new();
        variables.insert("number".to_owned(), "1".to_owned());
        variables.insert("number2".to_owned(), "2".to_owned());
        let file = load_file(Path::new("tests/templates/template_dir/file_1.txt"), &variables).unwrap();

        let expected_file = Path::new("tests/templates/template_dir_expected/file_1.txt");
        let expected_file = fs::read_to_string(expected_file).unwrap();
        assert_eq!(file, expected_file);
    }

    #[test]
    fn test_load_file_2() {
        let mut variables = HashMap::new();
        variables.insert("number".to_owned(), "1".to_owned());
        variables.insert("number2".to_owned(), "2".to_owned());
        let file = load_file(Path::new("tests/templates/template_dir/file_2.txt"), &variables).unwrap();

        let expected_file = Path::new("tests/templates/template_dir_expected/file_2.txt");
        let expected_file = fs::read_to_string(expected_file).unwrap();
        assert_eq!(file, expected_file);
    }

    #[test]
    fn test_load_files() {
        let mut variables = HashMap::new();
        variables.insert("number".to_owned(), "1".to_owned());
        variables.insert("number2".to_owned(), "2".to_owned());
        let files = load_files(Path::new("tests/templates/template_dir"), &variables).unwrap();

        let expected_file_1 = Path::new("tests/templates/template_dir_expected/file_1.txt");
        let expected_file_1 = fs::read_to_string(expected_file_1).unwrap();
        let expected_file_2 = Path::new("tests/templates/template_dir_expected/file_2.txt");
        let expected_file_2 = fs::read_to_string(expected_file_2).unwrap();

        assert_eq!(*files.get(Path::new("template_dir/file_1.txt")).unwrap(), expected_file_1);
        assert_eq!(*files.get(Path::new("template_dir/file_2.txt")).unwrap(), expected_file_2);
    }

    #[test]
    fn test_build_tempate() -> Result<()> {
        let config = Config {
            template_dir: Some(PathBuf::from("tests/templates")),

            ..Default::default()
        };
        let mut variables = HashMap::new();
        variables.insert("number".to_owned(), "1".to_owned());
        variables.insert("number2".to_owned(), "2".to_owned());
        let tempaltes = Template::load_templates(&config).unwrap();
        let template = tempaltes.get("test").unwrap();

        // Delete test directory if it exists
        let test_dir = Path::new("tests/test_files/template_test");
        if test_dir.exists() {
            fs::remove_dir_all(test_dir).unwrap();
        }

        template.build(test_dir.to_path_buf(), &mut variables, &config)?;

        Ok(())
    }
}
