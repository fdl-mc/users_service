//! Route handlers responses.

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}
