use anyhow::Result;
use rand::rngs::OsRng;
use rand::RngCore;
use secrecy::Secret;

use crate::auth::oidc::generate_kid;

pub struct Hs256 {
    kid: String,
    key: Secret<[u8; 32]>,
}

impl Hs256 {
    pub fn new() -> Result<Hs256> {
        let mut key = [0u8; 32];

        OsRng.try_fill_bytes(&mut key)?;

        Ok(Hs256 {
            kid: generate_kid()?,
            key: Secret::new(key),
        })
    }
}
