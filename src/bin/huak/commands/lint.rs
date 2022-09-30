use huak::errors::CliError;
use huak::ops;
use huak::{errors::CliResult, project::Project};
use std::env;
use std::process::ExitCode;

/// Run the `lint` command.
pub fn run() -> CliResult<()> {
    // This command runs from the context of the cwd.
    let cwd = env::current_dir()?;
    let project = match Project::from(cwd) {
        Ok(p) => p,
        Err(e) => return Err(CliError::new(e, ExitCode::FAILURE)),
    };

    if let Err(e) = ops::lint::lint_project(&project) {
        return Err(CliError::new(e, ExitCode::FAILURE));
    };

    Ok(())
}
