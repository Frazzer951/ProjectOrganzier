use crate::{
    config::Config,
    database::add_project,
    error::Error,
    project::Project,
    utils::{create_spinner, Result},
};
use clap::ArgMatches;
use console::Term;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Input;
use std::path::PathBuf;

struct NewParams {
    pub(crate) name: Option<String>,
    pub(crate) desc: Option<String>,
    pub(crate) tags: Vec<String>,
    pub(crate) language: Option<String>,
    pub(crate) category: Option<String>,
    pub(crate) templates: Vec<String>,
}

pub fn new(sub_matches: &ArgMatches, config: &mut Config) -> Result<()> {
    let dir = sub_matches.get_one::<PathBuf>("directory").cloned();
    let temp_dir = sub_matches.get_one::<PathBuf>("template-directory").cloned();
    let mut name = sub_matches.get_one::<String>("name").cloned();
    let mut desc = sub_matches.get_one::<String>("desc").cloned();
    let mut language = sub_matches.get_one::<String>("language").cloned();
    let mut category = sub_matches.get_one::<String>("category").cloned();
    let mut tags = sub_matches
        .get_many::<String>("tags")
        .into_iter()
        .flatten()
        .cloned()
        .collect::<Vec<_>>();
    let mut templates = sub_matches
        .get_many::<String>("templates")
        .into_iter()
        .flatten()
        .cloned()
        .collect::<Vec<_>>();
    let interactive = sub_matches.get_flag("interactive");

    if interactive {
        let new_params = new_params_interactive(name, desc, tags, language, category, templates)?;

        name = new_params.name;
        desc = new_params.desc;
        tags = new_params.tags;
        language = new_params.language;
        category = new_params.category;
        templates = new_params.templates;

        println!("\n\n");
        println!("Name: {name:?}");
        println!("Desc: {desc:?}");
        println!("Tags: {tags:?}");
        println!("Language: {language:?}");
        println!("Category: {category:?}");
        println!("Templates: {templates:?}");
    }

    if name.is_none() {
        println!("A name is required for a project, please specify one");
        return Ok(());
    }

    let mut project = Project::new(name, desc, tags, language, category);

    if let Some(temp_dir) = temp_dir {
        config.template_dir = Some(temp_dir.to_str().unwrap_or_default().to_owned());
    }

    let pb = create_spinner("Creating Folder...")?;

    match project.build(dir, config, templates) {
        Ok(_) => {},
        Err(e) => match e {
            Error::ConfigMissingValue(e) => {
                println!(
                    "Missing a value for `{e}`, either set it in the config, or pass a directory through the command line"
                );
                return Ok(());
            },
            e => return Err(e),
        },
    };
    pb.finish_with_message("Folder Created");

    add_project(config, &project)?;
    println!("{project:#?}");
    Ok(())
}

fn new_params_interactive(
    name: Option<String>,
    desc: Option<String>,
    mut tags: Vec<String>,
    language: Option<String>,
    category: Option<String>,
    mut templates: Vec<String>,
) -> Result<NewParams> {
    // Get Name
    let name = Some(
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Project Name")
            .with_initial_text(name.unwrap_or_default())
            .interact_text()?,
    );

    // Get Description
    let mut desc: Option<String> = Some(
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Project Desc")
            .with_initial_text(desc.unwrap_or_default())
            .allow_empty(true)
            .interact_text()?,
    );

    // Set to None if Empty
    if let Some(d) = &desc {
        if d.is_empty() {
            desc = None;
        }
    };

    // Get Tags
    let term = Term::stdout();
    loop {
        term.write_line(&format!("Current tags are: {tags:?}"))?;
        let tag: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Tags (leave empty to continue)")
            .allow_empty(true)
            .interact_text_on(&term)?;
        if tag.is_empty() {
            break;
        }
        tags.push(tag);
        tags.sort();
        term.clear_last_lines(2)?;
    }

    // Get Language
    let mut language: Option<String> = Some(
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Project Language")
            .with_initial_text(language.unwrap_or_default())
            .allow_empty(true)
            .interact_text()?,
    );
    // Set to none if empty
    if let Some(lang) = &language {
        if lang.is_empty() {
            language = None;
        }
    };

    // Get Category
    let mut category: Option<String> = Some(
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Project Category")
            .with_initial_text(category.unwrap_or_default())
            .allow_empty(true)
            .interact_text()?,
    );

    // Get to None if Empty
    if let Some(d) = &category {
        if d.is_empty() {
            category = None;
        }
    };

    // Get Templates
    let term = Term::stdout();
    loop {
        term.write_line(&format!("Current templates are: {templates:?}"))?;
        let template: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Templates (leave empty to continue)")
            .allow_empty(true)
            .interact_text_on(&term)?;
        if template.is_empty() {
            break;
        }
        templates.push(template);
        templates.sort();
        term.clear_last_lines(2)?;
    }

    Ok(NewParams {
        name,
        desc,
        tags,
        language,
        category,
        templates,
    })
}
