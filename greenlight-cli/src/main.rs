mod cli;

use clap::Parser;
use cli::{Args, Importance};
use greenlight_lib::{checks::Check, config::Config, errors::GreenlightError};
use std::collections::HashMap;
use std::{path::PathBuf, process::ExitCode};
use tracing::{debug, error, info};
use tracing_subscriber::fmt;

fn main() -> Result<ExitCode, GreenlightError> {
    fmt::init();
    info!("Starting Greenlight!");

    let args = Args::parse();

    let config_path = args
        .config_path
        .unwrap_or_else(|| PathBuf::from("/etc/greenlight/config.yaml"));

    let config: Config = Config::from_path(&config_path)?;

    // Merge required and wanted checks, required takes precedence
    let mut check_map: HashMap<Check, Importance> = HashMap::new();

    for check in config.wanted.checks {
        check_map.entry(check).or_insert(Importance::Wanted);
    }

    for check in config.required.checks {
        check_map.insert(check, Importance::Required); // Overwrites wanted
    }

    let checks_to_run: Vec<(Check, Importance)> = check_map.into_iter().collect();
    debug!(
        "Total checks to run ({}): {:?}",
        checks_to_run.len(),
        checks_to_run
    );

    let exit_code = ExitCode::SUCCESS;

    for (check, importance) in checks_to_run {
        info!("Running check: {:?}", check);
        match check.run() {
            Ok(true) => info!("✅ Check passed: {:?}", check),
            Ok(false) => match importance {
                Importance::Required => {
                    error!("❌ Required check failed: {:?}", check);
                    return Err(GreenlightError::CheckFailed(format!(
                        "{:?} did not pass validation",
                        check
                    )));
                }
                Importance::Wanted => {
                    error!("⚠️  Wanted check failed: {:?}", check);
                    // Continue
                }
                Importance::All => unreachable!(),
            },
            Err(e) => {
                error!("❌ Error running check: {:?}", e);
                return Err(e);
            }
        }
    }

    Ok(exit_code)
}
