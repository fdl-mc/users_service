//! Request body payloads.

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct FindPayload {
    pub nickname: String,
}

#[derive(Deserialize, Debug)]
pub struct ChangePasswordPayload {
    pub new_password: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateUserPayload {
    pub username: String,
    pub password: String,
}
