use crate::{
    types::types::{Data, Error},
    utils::env,
};
use anyhow::Result;
use poise::serenity_prelude as serenity;
use serenity::model::id::ChannelId;

pub async fn awake(
    _ctx: &serenity::Context,
    _event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
    data_about_bot: &serenity::Ready,
) -> Result<(), Error> {
    println!("Logged in as {}", data_about_bot.user.tag());
    /*
    rustical_message(
        _ctx,
        ChannelId::new(1160065321013620857), //bot
        //ChannelId::new(1120455140416172115), //genny
        env::laptop(),
    )
    .await?;
    */

    Ok(())
}

async fn rustical_message(
    ctx: &serenity::Context,
    c: ChannelId,
    laptop: String,
) -> Result<(), Error> {
    let message = "Ruststicks";

    let l: String = match laptop.parse().unwrap() {
        1 => " Laptopically :cold_face:".to_string(),
        2 => " Dockically :eagle:".to_string(),
        _ => "".to_string(),
    };

    let channel = c;
    let channel = channel
        .to_channel(&ctx.http)
        .await
        .expect("this channel will always work");
    if let Some(channel) = channel.guild() {
        channel.say(&ctx.http, message.to_string() + &l).await?;
    }
    Ok(())
}
