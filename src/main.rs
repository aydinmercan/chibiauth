mod config;
mod controller;

use anyhow::{bail, Result};
use clap::Parser;
use rusqlite::Connection;

use crate::config::read_to_config;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    pub config: String,
}

fn setup_db_conn_and_for_replication(path: String) -> Result<Connection> {
    let db = Connection::open(path)?;

    let mode: String = db.pragma_update_and_check(None, "journal_mode", "WAL", |row| row.get(0))?;

    if mode != "wal" {
        bail!("couldn't set journaling to WAL");
    }

    db.pragma_update(None, "busy_timeout", "5000")?;
    db.pragma_update(None, "synchronous", "NORMAL")?;

    Ok(db)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let config = read_to_config(args.config)?;

    let db = setup_db_conn_and_for_replication(config.database)?;

    Ok(())
}
