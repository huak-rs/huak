use std::fs;

use super::project_utils;
use crate::project::Project;

pub fn create_project_toml(project: &Project) -> Result<(), anyhow::Error> {
    let toml = project_utils::create_toml(project)?;

    if project.root.join("pyproject.toml").exists() {
        return Err(anyhow::format_err!("a pyproject.toml already exists"));
    }

    // Serialize pyproject.toml.
    fs::write(&project.root.join("pyproject.toml"), toml.to_string()?)?;
    Ok(())
}
