use crate::{
    env::python::PythonEnvironment,
    errors::{CliError, CliResult},
    project::{python::PythonProject, Project},
};

/// Format Python code from the project's root.
pub fn fmt_project(project: &Project, is_check: &bool) -> CliResult {
    let venv = match project.venv() {
        Some(v) => v,
        None => {
            return Err(CliError::new(anyhow::format_err!("invalid venv"), 2))
        }
    };

    match is_check {
        true => venv.exec_module(
            "black",
            &[".", "--line-length", "79", "--check"],
            &project.root,
        )?,
        false => venv.exec_module(
            "black",
            &[".", "--line-length", "79"],
            &project.root,
        )?,
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use tempfile::tempdir;

    use super::*;

    use crate::utils::test_utils::{copy_dir, create_mock_project};

    #[test]
    fn fmt() {
        let directory = tempdir().unwrap().into_path().to_path_buf();
        let from_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources")
            .join("mock-project");

        copy_dir(&from_dir, &directory);

        let project_path = directory.join("mock-project");
        let project = create_mock_project(project_path.clone()).unwrap();
        let venv = project.venv();
        if let Some(v) = venv {
            v.exec_module("pip", &["install", "black"], &project.root)
                .unwrap();
        }

        let fmt_filepath = project
            .root
            .join("src")
            .join("mock_project")
            .join("fmt_me.py");
        let pre_fmt_str = r#"""
def fn( ):
    pass"#;
        fs::write(&fmt_filepath, pre_fmt_str).unwrap();
        fmt_project(&project, &false).unwrap();
        let post_fmt_str = fs::read_to_string(&fmt_filepath).unwrap();

        assert_ne!(pre_fmt_str, post_fmt_str);
    }
}
