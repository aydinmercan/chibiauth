use std::convert::AsRef;
use std::ops::{Deref, Drop};
use std::option::Option;
use std::path::Path;
use std::sync::Arc;
use std::vec::Vec;

use anyhow::{bail, Result};
use rusqlite::Connection;
use tokio::sync::Mutex;

static SCHEMA: &str = include_str!("../schema.sql");

pub struct PooledConnection {
    conn: Option<Connection>,
    pool: Arc<Mutex<Vec<Connection>>>,
}

/// Naive databse pool.
#[derive(Clone)]
pub struct Database {
    pool: Arc<Mutex<Vec<Connection>>>,
}

/// Requires WAL and sets up complementary settings.
pub fn setup_sqlite_conn(path: &dyn AsRef<Path>) -> Result<Connection> {
    let conn = Connection::open(path)?;

    conn.pragma_update(None, "journal_mode", "wal")?;
    conn.pragma_update(None, "synchronous", "normal")?;
    conn.pragma_update(None, "busy_timeout", 5000)?;

    conn.execute_batch(SCHEMA)?;

    Ok(conn)
}

impl Deref for PooledConnection {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        self.conn.as_ref().unwrap()
    }
}

impl Drop for PooledConnection {
    fn drop(&mut self) {
        let mut p = self.pool.blocking_lock();

        p.push(self.conn.take().unwrap());
    }
}

impl Database {
    pub fn new(path: &dyn AsRef<Path>, capacity: usize) -> Result<Database> {
        let mut v = Vec::new();

        v.try_reserve_exact(capacity)?;

        for _ in 0..capacity {
            let c = setup_sqlite_conn(path)?;

            v.push(c);
        }

        let pool = Arc::new(Mutex::new(v));

        Ok(Database { pool })
    }

    pub async fn get(&mut self) -> Result<PooledConnection> {
        let mut pool = self.pool.lock().await;

        let conn = pool.pop();

        Ok(PooledConnection {
            conn,
            pool: Arc::clone(&self.pool),
        })
    }
}
