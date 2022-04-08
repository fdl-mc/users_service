pub mod config;
pub mod migration;
pub mod prelude;

use rand::{distributions::Alphanumeric, Rng};
use sha2::{Digest, Sha256};

pub fn hash_password(password: String, salt: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    hasher.update(salt);
    format!("{:x}", hasher.finalize())
}

pub fn generate_salt() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}
