use crate::{
    events::{
        awake::awake, 
        message::message
    },
    types::types::{Data, Error}
};
use anyhow::Result;
use poise::serenity_prelude as serenity;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            awake(ctx, event, _framework, data, data_about_bot).await?;
        }
        // me when the
        serenity::FullEvent::Message { new_message } => {
            message(ctx, event, _framework, data, new_message).await?;
        }
        _ => {}
    };
    Ok(())
}
