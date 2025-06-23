use std::path::PathBuf;

use clap::Parser;

/// This program acts as a proxy for OpenRouter, allowing the use of multiple
/// free OpenRouter accounts to handle requests. It automatically rotates
/// between the available accounts, prioritizing those that have made the
/// fewest requests today. This helps avoid exceeding daily usage limits for
/// any individual account.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    /// Path to configuration file
    #[arg(short, long, default_value = "config.toml")]
    pub(crate) config: PathBuf,
}
