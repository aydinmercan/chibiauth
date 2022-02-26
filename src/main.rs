mod controller;
mod database;

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use parking_lot::RwLock;

use crate::controller::setup_router;

pub type CsrfMap = RwLock<HashMap<String, String>>;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    pub database: String,

    #[clap(short, long)]
    pub port: u16,
}

fn setup_default_subscriber() -> Result<()> {
    tracing_subscriber::fmt::init();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    setup_default_subscriber()?;

    let csrf_map = Arc::new(RwLock::new(HashMap::<String, String>::new()));

    let db = crate::database::setup_connection_and_for_replication(args.database)?;

    let router = setup_router(csrf_map);

    Ok(())
}
