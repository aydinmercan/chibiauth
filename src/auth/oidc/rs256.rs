use anyhow::Result;
use base64ct::{Base64Url, Encoding};
use rand::rngs::OsRng;
use rand::RngCore;
use rsa::pkcs1v15::{SigningKey, VerifyingKey};
use rsa::{PublicKeyParts, RsaPrivateKey, RsaPublicKey};
use serde_json::json;
use sha2::Sha256;

use crate::auth::oidc::generate_kid;

static RSA_BITS: usize = 2048;

/// RSASSA-PKCS1v1.5 with SHA-2-256
///
/// Only 2048-bit keys are used.
pub struct Rs256 {
    kid: String,
    signing_key: SigningKey<Sha256>,
    verifying_key: VerifyingKey<Sha256>,
}

impl Rs256 {
    pub fn new() -> Result<Rs256> {
        let kid = generate_kid()?;

        let p = RsaPrivateKey::new(&mut OsRng, RSA_BITS)?;

        let signing_key = SigningKey::new_with_prefix(p);

        let verifying_key = VerifyingKey::from(&signing_key);

        Ok(Rs256 {
            kid,
            signing_key,
            verifying_key,
        })
    }

    fn discovery_key(&self) -> Result<serde_json::Value> {
        let public_key: &RsaPublicKey = self.verifying_key.as_ref();

        let n_bytes = public_key.n().to_bytes_be();

        let n = Base64Url::encode_string(&n_bytes);

        Ok(json!({
            "alg": "RS256",
            "kty": "RSA",
            "kid": self.kid,
            "e": "AQAB",
            "n": n,
        }))
    }
}

#[cfg(test)]
mod tests {
    use rsa::{PublicKeyParts, RsaPublicKey};

    use crate::auth::oidc::rs256::*;

    // This can fail in two situations.
    // 1. BigUint no longer uses the minimum bits required.
    // 2. The `rsa` crate has changed its default exponent.
    #[test]
    fn rsa_default_exp_is_serialized_as_aqab() {
        let private_key = RsaPrivateKey::new(&mut OsRng, RSA_BITS).unwrap();

        let public_key = RsaPublicKey::from(&private_key);

        let e_bytes = public_key.e().to_bytes_be();

        let e_str = Base64Url::encode_string(&e_bytes);

        assert_eq!(e_str, "AQAB");
    }
}
