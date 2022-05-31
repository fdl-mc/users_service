use rand::{distributions::Alphanumeric, Rng};
use sha2::{Digest, Sha256};

/// Hash a password using SHA256.
pub fn hash_password(password: String, salt: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password);
    hasher.update(salt);
    format!("{:x}", hasher.finalize())
}

/// Generate a random salt (alphanumeric 32 chars string).
pub fn generate_salt() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}
