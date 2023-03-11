use crate::{utils::Result, config::Config, database::load_projects};
use prettytable::{format, row, Table};

pub fn list(config: &Config) -> Result<()> {
    let projects = load_projects(config)?;
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.set_titles(row!["Name", "Description", "Directory"]);
    for project in projects {
        table.add_row(row![
            project.name.unwrap_or_default(),
            project.desc.unwrap_or_default(),
            project.directory.unwrap_or_default().display()
        ]);
    }
    table.printstd();
    Ok(())
}
