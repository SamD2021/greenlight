mod cli;
use clap::Parser;
use cli::Args;
use greenlight_lib::{checks::Check, config::Config, errors::GreenlightError};
use std::{collections::HashSet, path::PathBuf, process::ExitCode};
use tracing::{debug, error, info};
use tracing_subscriber::fmt;
fn main() -> Result<ExitCode, GreenlightError> {
    fmt::init(); // Setup logging once!
    info!("Starting Greenlight!");
    let args = Args::parse();
    let config_path = match args.config_path {
        Some(config_path) => config_path,
        None => PathBuf::from("/etc/greenlight/config.yaml"),
    };
    let config: Config = Config::from_path(&config_path)?;

    let included_checks: HashSet<Check> = args
        .include_checks
        .or_else(|| config.checks.include.clone()) // clone because Option<Vec> needs to be moved
        .unwrap_or_default()
        .into_iter()
        .collect();
    let excluded_checks: HashSet<Check> = args
        .exclude_checks
        .or_else(|| config.checks.exclude.clone())
        .unwrap_or_default()
        .into_iter()
        .collect();
    let target = args
        .target
        .or_else(|| Some(config.system.target().clone()))
        .unwrap_or_default();
    let total_checks: HashSet<_> = included_checks
        .union(&target.default_checks())
        .filter(|c| !excluded_checks.contains(c))
        .cloned()
        .collect();
    debug!("Total checks ({}): {:?} ", total_checks.len(), total_checks);
    for check in total_checks {
        info!("Running check: {:?}", check);
        match check.run() {
            Ok(passed) => {
                if passed {
                    info!("✅ Check passed: {:?}", check);
                } else {
                    error!("❌ Check failed: {:?}", check);
                    return Err(GreenlightError::CheckFailed(format!(
                        "{:?} did not pass validation",
                        check
                    )));
                }
            }
            Err(check_error) => {
                error!("❌ Check execution error: {:?}", check_error);
                return Err(check_error);
            }
        }
    }

    Ok(ExitCode::SUCCESS)
}
