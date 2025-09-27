use crate::utils::env;
use poise::serenity_prelude::{self as serenity, Http};
use serenity::model::id::ChannelId;

pub fn send_message_in_channel(message: &str, channel_id: u64) {
    let channel = ChannelId::new(channel_id);
    let http = Http::new(&env::discord_token());

    // Spawn a new Tokio task to send the message asynchronously
    let message = message.to_string();
    tokio::spawn(async move {
        let _ = channel.say(&http, format!("{}", message)).await;
    });
}
