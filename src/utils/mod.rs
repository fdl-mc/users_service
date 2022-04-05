pub mod config;
pub mod migration;
pub mod prelude;

use sha2::{Digest, Sha256};

pub fn hash_password(password: String, salt: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    hasher.update(salt);
    format!("{:x}", hasher.finalize())
}
