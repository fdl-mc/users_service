use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub postgres_url: String,
    pub jwt_secret: String,
}
