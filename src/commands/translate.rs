use crate::types::types::{AppContext, Error};
use serenity::builder::GetMessages;

#[poise::command(slash_command)]
pub async fn translate(ctx: AppContext<'_>) -> Result<(), Error> {
    //print the 3 most recent messages in the channel to the console
    let channel_id = ctx.channel_id();

    let builder = GetMessages::new().limit(3);
    let messages = channel_id.messages(ctx.http(), builder).await?;

    for message in messages.iter() {
        println!("{}: {}", message.author.name, message.content);
    }
    Ok(())
}
