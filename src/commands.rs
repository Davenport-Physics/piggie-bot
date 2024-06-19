

use poise::Framework;

use crate::config::CONFIG;

pub struct Data;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub fn init_slash_commands() -> Framework<Data, Error> {

    poise::Framework::builder()
        .options(poise::FrameworkOptions 
            { 
                commands: vec![], 
                ..Default::default()
            }
        )
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {

                if cfg!(debug_assertions) {
                    poise::builtins::register_in_guild(ctx, &framework.options().commands, CONFIG.discord_guild_id.into()).await?;
                } else {
                    poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                }

                Ok(Data {})
                
            })
        })
        .build()

}