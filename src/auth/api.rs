use anyhow::Result;
use blake2::{Blake2b512, Blake2s256, Digest};
use rand::rngs::OsRng;
use rand::RngCore;
use rusqlite::Connection;
use secrecy::{ExposeSecret, Secret, SecretString};

pub struct ApiKey {
    selector: String,
    verifier: String,
}

impl ApiKey {
    pub fn new() -> Result<SecretString, ApiKey> {
        let mut raw_key = [0u8; 32];

        OsRng.try_fill_bytes(&mut key)?;

        let key = base16ct::upper::encode_string(raw_key);

        let mut h = Blake2b512::new();

        h.update(b"");
    }
}
