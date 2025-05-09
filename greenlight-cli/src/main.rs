mod cli;

use clap::Parser;
use cli::{Args, Importance};
use futures::future::join_all;
use greenlight_lib::{checks::Check, config::Config, errors::GreenlightError};
use std::collections::HashMap;
use std::{path::PathBuf, process::ExitCode};
use tracing::{debug, error, info, span, Level};
use tracing_futures::Instrument;
use tracing_subscriber::{fmt::Subscriber, EnvFilter};

#[tokio::main]
async fn main() -> Result<ExitCode, GreenlightError> {
    let args = Args::parse();

    let config_path = args
        .config_path
        .unwrap_or_else(|| PathBuf::from("/etc/greenlight/config.toml"));

    let config: Config = Config::from_path(&config_path)?;

    // Map log level from config to `tracing::Level`
    let level = match config.logging {
        greenlight_lib::config::Logging::Basic { level, .. } => match level {
            greenlight_lib::config::LogLevel::Debug => "debug",
            greenlight_lib::config::LogLevel::Info => "info",
            greenlight_lib::config::LogLevel::Warn => "warn",
            greenlight_lib::config::LogLevel::Error => "error",
        },
        _ => "info", // fallback
    };

    // Now initialize tracing with the level from config
    Subscriber::builder()
        .with_env_filter(EnvFilter::new(level))
        .init();
    info!("Starting Greenlight!");

    // Merge required and wanted checks
    let mut check_map: HashMap<Check, Importance> = HashMap::new();
    for check in config.wanted.checks {
        check_map.entry(check).or_insert(Importance::Wanted);
    }
    for check in config.required.checks {
        check_map.insert(check, Importance::Required);
    }

    let checks_to_run: Vec<(Check, Importance)> = check_map
        .into_iter()
        .filter(|(_, importance)| match args.only {
            Importance::All => true,
            _ => *importance == args.only,
        })
        .collect();

    debug!(
        "Total checks to run ({}): {:?}",
        checks_to_run.len(),
        checks_to_run
    );

    let results = join_all(checks_to_run.into_iter().map(|(check, importance)| {
        let span = span!(Level::INFO, "check", check = ?check);
        async move {
            info!("Running check");
            let result = check.run().await;
            (check, importance, result)
        }
        .instrument(span)
    }))
    .await;
    let mut any_wanted_failed = false;

    for (check, importance, result) in results {
        match result {
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
                    any_wanted_failed = true;
                }
                Importance::All => unreachable!(),
            },
            Err(e) => {
                error!("❌ Error running check: {:?}", e);
                return Err(e);
            }
        }
    }
    if any_wanted_failed {
        return Err(GreenlightError::CheckFailed(
            "At least one wanted check failed".to_string(),
        ));
    }

    Ok(ExitCode::SUCCESS)
}
