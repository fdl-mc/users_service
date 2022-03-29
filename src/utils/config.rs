use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub postgres_url: String,
    pub discord_client_id: String,
    pub discord_client_secret: String,
    pub discord_redirect_uri: String,
    pub jwt_secret: String,
}
