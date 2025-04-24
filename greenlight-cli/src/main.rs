mod cli;
use clap::Parser;
use cli::Args;
use greenlight_lib::{config::Config, errors::GreenlightError};
use std::{path::PathBuf, process::ExitCode};
fn main() -> Result<ExitCode, GreenlightError> {
    let args = Args::parse();
    let config_path = match args.config_path {
        Some(config_path) => config_path,
        None => PathBuf::from("/etc/greenlight/config.yaml"),
    };
    let config: Config = Config::from_path(&config_path)?;

    let Some(checks) = args.checks else {
        let Some(checks) = config.checks else {
            return Err(GreenlightError::NoChecksProvided);
        };
        checks
    };

    Ok(ExitCode::SUCCESS)
}
