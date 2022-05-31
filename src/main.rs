mod controller;
mod database;
mod engine;
mod server;

use std::collections::HashMap;
use std::option::Option;
use std::process::{ExitCode, Termination};
use std::sync::RwLock;

use anyhow::Result;
use clap::{Parser, Subcommand};

pub type CsrfMap = RwLock<HashMap<String, String>>;

#[repr(u8)]
enum ServerResult {
    Success = 0,
    Failure = 1,
}

impl Termination for ServerResult {
    fn report(self) -> ExitCode {
        ExitCode::from(self as u8)
    }
}

#[derive(Parser)]
#[clap(version)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Start the server
    Run {
        #[clap(short, long)]
        database: String,

        #[clap(short, long)]
        url: String,

        #[clap(short, long)]
        port: u16,
    },

    /// Initialize the database and/or regenerate public identities
    Init {
        #[clap(short, long)]
        database: String,
    },

    /// Control API keys
    Api {
        #[clap(short, long)]
        database: String,

        #[clap(short, long)]
        generate: bool,

        #[clap(short, long)]
        revoke: Option<String>,

        #[clap(short = 'R', long)]
        revoke_all: bool,
    },
}

fn setup_default_subscriber() -> Result<()> {
    tracing_subscriber::fmt().pretty().init();

    std::panic::set_hook(Box::new(|panic| {
        if let Some(location) = panic.location() {
            tracing::error!(
                message = %panic,
                panic.file = location.file(),
                panic.line = location.line(),
                panic.column = location.column(),
            );
        } else {
            tracing::error!(message = %panic);
        }
    }));

    Ok(())
}

fn main() -> ServerResult {
    if setup_default_subscriber().is_err() {
        return ServerResult::Failure;
    }

    let args = Args::parse();

    match args.command {
        Command::Init { database } => {}
        Command::Run {
            database,
            port,
            url,
        } => {}
        Command::Api {
            database,
            generate,
            revoke,
            revoke_all,
        } => {}
    }

    ServerResult::Success
}
