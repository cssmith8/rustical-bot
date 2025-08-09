use crate::types::types::{Context, Error};
use anyhow::Result;

#[poise::command(slash_command, prefix_command)]
pub async fn say(
    ctx: Context<'_>,
    #[description = "Message to say"] message: String,
) -> Result<(), Error> {
    ctx.say(message).await?;
    Ok(())
}
