
use lazy_static::lazy_static;
use serde::Deserialize;

use web_base::config::ProjectSettings;

#[derive(Deserialize)]
pub struct Config {
    pub discord_application_id: String,
    pub discord_public_key: String,
    pub discord_token: String,
    pub discord_guild_id: u64
}

impl ProjectSettings<Config> for Config {}

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}