
use serenity::prelude::*;

pub mod config;
pub mod commands;

use config::CONFIG;

#[tokio::main]
async fn main() {

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = commands::init_slash_commands();

    let mut client =
        Client::builder(&CONFIG.discord_token, intents)
        .framework(framework)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
    
}