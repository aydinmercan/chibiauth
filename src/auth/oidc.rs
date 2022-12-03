//! OIDC
//!
//! # JWK
//! Only `HS256` and `RS256` are supported.

mod hs256;
mod rs256;

use std::collections::HashMap;

use anyhow::Result;
use base64ct::{Base64Url, Encoding};
use rand::rngs::OsRng;
use rand::RngCore;

pub use hs256::Hs256;
pub use rs256::Rs256;

pub struct OidcAuthnRequest {
    pub user: String,
    pub pass: String,
    pub redirect_uri: String,
}

pub struct OidcDiscovery {
    pub jwks: String,
}

pub struct OidcKeyEngine {
    discovery: OidcDiscovery,
}

/// Generates a 192-bit random key ID.
#[inline(always)]
fn generate_kid() -> Result<String> {
    let mut kid_bytes = [0u8; 24];

    OsRng.try_fill_bytes(&mut kid_bytes)?;

    Ok(Base64Url::encode_string(&kid_bytes))
}

impl OidcDiscovery {}
