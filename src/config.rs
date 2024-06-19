
use rand::Rng;

use lazy_static::lazy_static;
use serde::Deserialize;

use web_base::config::ProjectSettings;

#[derive(Deserialize)]
pub struct Config {
    pub discord_application_id: u64,
    pub discord_public_key: String,
    pub discord_token: String,
    pub discord_guild_id: u64,
    pub rustle_bag_responses: Vec<String>

}

impl ProjectSettings<Config> for Config {}

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

impl Config {

    pub fn get_rustle_bag_response(&self) -> String {

        let idx = rand::thread_rng()
            .gen_range(0..self.rustle_bag_responses.len());

        self.rustle_bag_responses[idx].clone()

    }

}