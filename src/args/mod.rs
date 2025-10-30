use std::{net::IpAddr, path::PathBuf};

use clap::Parser;

/// This program acts as a proxy for OpenRouter, allowing the use of multiple
/// free OpenRouter accounts to handle requests. It automatically rotates
/// between the available accounts, prioritizing those that have made the
/// fewest requests today. This helps avoid exceeding daily usage limits for
/// any individual account.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    /// Path to tokens file
    #[arg(short, long, default_value = "tokens.csv")]
    pub(crate) tokens: PathBuf,

    /// Path to tokens file
    #[arg(long, default_value = "allow.txt")]
    pub(crate) allow: PathBuf,

    /// Listening address
    #[arg(short, long, default_value = "127.0.0.1")]
    pub(crate) address: IpAddr,

    /// Listening port
    #[arg(short, long, default_value_t = 3333)]
    pub(crate) port: u16,
}
