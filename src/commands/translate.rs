use crate::types::types::{AppContext, Error};
use serenity::builder::GetMessages;
use serenity::model::id::{ChannelId, MessageId};

#[poise::command(slash_command)]
pub async fn translate(ctx: AppContext<'_>) -> Result<(), Error> {
    //print the 3 most recent messages in the channel to the console
    let channel_id = ctx.channel_id();

    let builder = GetMessages::new()
        .after(MessageId::new(158339864557912064))
        .limit(25);
    let _messages = channel_id.messages(&http, builder).await?;

    for message in _messages.iter() {
        println!("{}: {}", message.author.name, message.content);
    }
    Ok(())
}
