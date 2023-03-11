use crate::{config::Config, project::Project, utils};
use fs_err as fs;
use std::path::PathBuf;
use std::sync::Once;
use turbosql::{select, set_db_path, Turbosql};

static DB_INIT: Once = Once::new();

fn set_db(config: &Config) -> utils::Result<()> {
    let path = PathBuf::from(&config.database_path);

    let mut dir = path.clone();
    dir.pop();

    fs::create_dir_all(dir)?;

    DB_INIT.call_once(|| {
        match set_db_path(path.as_path()) {
            Ok(_) => {},
            Err(e) => panic!("{e}"),
        };
    });
    Ok(())
}

pub fn add_project(config: &Config, project: &Project) -> utils::Result<()> {
    set_db(config)?;

    project.insert()?;

    Ok(())
}

pub fn load_projects(config: &Config) -> utils::Result<Vec<Project>> {
    set_db(config)?;

    let projects = select!(Vec<Project>);

    match projects {
        Ok(projects) => Ok(projects),
        Err(e) => Err(crate::error::Error::Sql(e)),
    }
}

pub fn reset_database(config: &Config) -> utils::Result<()> {
    fs::remove_file(&config.database_path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use anyhow::Result;

    #[test]
    fn test_add() -> Result<()> {
        let config = Config {
            database_path: "./tests/test_files/database.db".to_owned(),
            base_dir: None,
        };

        let _ = std::fs::remove_file(&config.database_path);

        let project = Project {
            rowid: None,
            name: Some("Test".to_owned()),
            desc: Some("a test project".to_owned()),
            directory: Some(PathBuf::from("C:\\dev\\rust\\fpm")),
            tags: Some(vec!["test".to_owned(), "project".to_owned()]),
            language: Some("rust".to_owned()),
            category: None,
        };

        println!("Adding project");
        add_project(&config, &project)?;

        println!("Loading project");
        let projects = load_projects(&config)?;

        assert_eq!(projects.len(), 1);

        let p = projects[0].clone();

        assert_eq!(project.name, p.name);
        assert_eq!(project.desc, p.desc);
        assert_eq!(project.directory, p.directory);
        assert_eq!(project.tags, p.tags);
        assert_eq!(project.language, p.language);
        assert_eq!(project.category, p.category);

        Ok(())
    }
}
