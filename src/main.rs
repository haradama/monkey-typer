mod prelude;
mod errors;
mod logging;
mod config;

use crate::prelude::*;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "monkey-typer", about = "Live coding air-typing performer", version)]
struct Cli {
    #[arg(short = 'c', long = "config")]
    config: Option<PathBuf>,

    #[arg(long = "log-level")]
    log_level: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Doctor,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let cfg = config::AppConfig::load_from(cli.config.as_deref())?;

    if let Some(level) = cli.log_level.as_ref().or(cfg.log_level.as_ref()) {
        std::env::set_var("RUST_LOG", level);
    }

    logging::init();

    info!("monkey-typer starting up...");
    debug!("cli = {:?}", &cli);

    match cli.command.unwrap_or(Commands::Doctor) {
        Commands::Doctor => {
            info!("OK: CLI/Logging/Config is started.");
            println!("monkey-typer is ready. Try: `RUST_LOG=debug monkey-typer --help`");
        }
    }

    Ok(())
}