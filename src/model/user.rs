use anyhow::Result;
use argon2::password_hash::PasswordHash;
use argon2::{Algorithm, Argon2, PasswordHasher, PasswordVerifier, Version};
use password_hash::SaltString;
use rand::rngs::OsRng;
use secrecy::SecretString;
use uuid::Uuid;

// Parameters are from https://twitter.com/Sc00bzT/status/1557495201064558592
static ARGON2_M: u32 = 256;
static ARGON2_T: u32 = 8;
static ARGON2_P: u32 = 1;
static ARGON2_OUTPUT_LEN: Option<usize> = Some(32);

pub struct User {
    uuid: Uuid,
    username: String,
    secrethash: String,
}

#[inline(always)]
fn get_passphrase_kdf<'a>() -> Result<Argon2<'a>> {
    let params = argon2::Params::new(256, 8, 1, Some(32))?;

    let kdf = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    Ok(kdf)
}

impl User {
    pub fn new(username: String, secret: &[u8]) -> Result<User> {
        let params = argon2::Params::new(ARGON2_M, ARGON2_T, ARGON2_P, ARGON2_OUTPUT_LEN)?;
        let kdf = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

        let salt = SaltString::generate(&mut OsRng);

        let secrethash = kdf.hash_password(secret, &salt)?.to_string();

        Ok(User {
            uuid: Uuid::new_v4(),
            username,
            secrethash,
        })
    }

    pub fn regenerate_id(self) -> User {
        User {
            uuid: Uuid::new_v4(),
            username: self.username,
            secrethash: self.secrethash,
        }
    }

    pub fn verify(self, pass: &[u8]) -> Result<bool> {
        let kdf = get_passphrase_kdf()?;

        let parsed_hash = PasswordHash::new(&self.secrethash)?;

        match kdf.verify_password(pass, &parsed_hash) {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
}
