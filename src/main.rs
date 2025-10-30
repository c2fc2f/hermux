mod args;
mod config;
mod openrouter;

use std::fs::read_to_string;

use actix_web::{App, HttpServer};
use anyhow::Context;
use clap::Parser;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let args: args::Args = args::Args::parse();
    let config: config::Config = toml::from_str(
        &read_to_string(args.config).context("Cannot read the config file")?,
    )
    .context("Invalid config file")?;

    HttpServer::new(|| App::new().configure(openrouter::config))
        .bind((config.server.address, config.server.port))?
        .run()
        .await?;

    Ok(())
}
