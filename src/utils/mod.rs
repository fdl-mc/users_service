pub mod config;
pub mod prelude;

use sha2::{Sha256, Digest};

pub fn hash_password(password: String, salt: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    hasher.update(salt);
    format!("{:x}", hasher.finalize())
}
