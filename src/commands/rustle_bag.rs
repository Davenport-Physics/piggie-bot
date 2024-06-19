

use crate::{commands::{Context, Error}, config::CONFIG};

/// Don't do it!
#[poise::command(prefix_command,slash_command)]
pub async fn rustle_bag(ctx: Context<'_>,) -> Result<(),  Error> {

    ctx.say(CONFIG.get_rustle_bag_response()).await?;
    Ok(())

}