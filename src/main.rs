mod auth;
mod config;
mod database;
mod model;
mod state;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use tracing::{error, Level};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the server
    #[command(arg_required_else_help = true)]
    Run {
        /// Path to the configuration file
        #[arg(short, long)]
        config: PathBuf,

        /// Emit logs as ND-JSON
        #[arg(short, long)]
        json: bool,

        /// Repeat to increase verbosity up to 2 times
        #[arg(short, long, action = clap::ArgAction::Count)]
        verbose: u8,
    },

    #[command(arg_required_else_help = true)]
    Keygen {
        /// Path to the configuration file
        #[arg(short, long)]
        config: PathBuf,

        #[arg(long)]
        api: bool,

        #[arg(long)]
        encrypt: bool,

        #[arg(long)]
        oidc_rs256: bool,
    },

    #[command(arg_required_else_help = true)]
    Gc {
        /// Path to the configuration file
        #[arg(short, long)]
        config: PathBuf,
    },

    Revoke {
        /// Path to the configuration file
        #[arg(short, long)]
        config: PathBuf,
    },
}

fn setup_logging(use_json: bool, verbosity: u8) -> Result<()> {
    let max_level = match verbosity {
        0 => Level::INFO,
        1 => Level::DEBUG,
        2.. => Level::TRACE,
    };

    let fmt = tracing_subscriber::fmt().with_max_level(max_level);

    if use_json {
        let collector = fmt.json().finish();
        tracing::subscriber::set_global_default(collector)?;
    } else {
        let collector = fmt.pretty().finish();
        tracing::subscriber::set_global_default(collector)?;
    }

    std::panic::set_hook(Box::new(|panic| {
        if let Some(location) = panic.location() {
            error!(
                message = %panic,
                panic.file = location.file(),
                panic.line = location.line(),
                panic.column = location.column(),
            );
        } else {
            error!(message = %panic);
        }
    }));

    Ok(())
}

pub fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Run {
            config,
            json,
            verbose,
        } => {
            setup_logging(json, verbose);
        }
        Commands::Keygen {
            config,
            api,
            encrypt,
            oidc_rs256,
        } => {}
        _ => {}
    }
}
