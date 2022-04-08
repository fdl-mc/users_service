use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct FindData {
    pub nickname: String,
}

#[derive(Deserialize, Debug)]
pub struct ChangePasswordData {
    pub new_password: String,
}
