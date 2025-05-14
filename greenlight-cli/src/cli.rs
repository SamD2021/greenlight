use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    /// Path to Greenlight config YAML file
    #[arg(short = 'f', long)]
    pub config_path: Option<PathBuf>,

    /// Path to Greenlight config YAML file
    #[cfg(feature = "plugins")]
    #[arg(short = 'p', long)]
    pub plugin_dir: Option<PathBuf>,

    /// Run only this importance level of checks: required, wanted, or all
    #[arg(
        long,
        value_enum,
        default_value = "all",
        help = "Run only checks marked as this importance level (required, wanted, all) [default: all]"
    )]
    pub only: Importance,
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
#[clap(rename_all = "lowercase")]
pub enum Importance {
    Required,
    Wanted,
    All,
}
