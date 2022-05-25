#[derive(serde::Deserialize, Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
}
