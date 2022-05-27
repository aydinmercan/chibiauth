use anyhow::{bail, Result};
use tracing::{debug, error, info, warn};

use rusqlite::Connection;

pub fn setup_connection_and_for_replication(path: String) -> Result<Connection> {
    debug!(target = ?path, "opening database");

    let db = Connection::open(path)?;

    let mode: String = db.pragma_update_and_check(None, "journal_mode", "WAL", |row| row.get(0))?;

    if mode != "wal" {
        bail!("couldn't set journaling to WAL");
    }

    let secure_delete: u64 = db.pragma_query_value(None, "secure_delete", |row| row.get(0))?;

    if secure_delete == 0 {
        warn!("secure delete is off, beware of lingering secrets in the database");
    }

    db.pragma_update(None, "busy_timeout", "5000")?;
    db.pragma_update(None, "synchronous", "NORMAL")?;
    db.pragma_update(None, "foreign_keys", "ON")?;

    db.execute_batch(include_str!("../schema.sql"))?;

    Ok(db)
}
