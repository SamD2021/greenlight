mod cli;
use clap::Parser;
use cli::Args;
use greenlight_lib::{config::Config, errors::GreenlightError};
use std::{collections::HashSet, path::PathBuf, process::ExitCode};
fn main() -> Result<ExitCode, GreenlightError> {
    let args = Args::parse();
    let config_path = match args.config_path {
        Some(config_path) => config_path,
        None => PathBuf::from("/etc/greenlight/config.yaml"),
    };
    let config: Config = Config::from_path(&config_path)?;

    let included_checks = args
        .include_checks
        .or_else(|| {
            if config.checks.include.is_empty() {
                None
            } else {
                Some(config.checks.include.into_iter().collect())
            }
        })
        .unwrap_or_default();
    let excluded_checks = args
        .exclude_checks
        .or_else(|| {
            if config.checks.exclude.is_empty() {
                None
            } else {
                Some(config.checks.exclude.into_iter().collect())
            }
        })
        .unwrap_or_default();

    Ok(ExitCode::SUCCESS)
}
