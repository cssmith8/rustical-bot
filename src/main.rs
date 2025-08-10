use crate::types::types::{Data, Error};
use anyhow::Result;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use poise::serenity_prelude as serenity;
use serenity::prelude::*;
use serenity::Client;
use std::env;

mod commands;
mod events;
mod types;
mod utils;

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            events::on_awake::on_awake(ctx, event, _framework, data, data_about_bot).await?;
        }
        // me when the
        serenity::FullEvent::Message { new_message } => {
            events::on_message::on_message(ctx, event, _framework, data, new_message).await?;
        }
        _ => {}
    };
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected discord token env");

    let db = PickleDb::load(
        "data/real.db",
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    );

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::say::say(),
                //commands::modal::modal(),
                commands::joke::joke(),
                commands::remark::remark(),
                commands::translate::translate(),
                commands::logs::logs(),
            ],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    db: Mutex::new(db?),
                })
            })
        })
        .build();

    let mut client = Client::builder(token, intents)
        .framework(framework)
        .await
        .expect("Could not create client");

    if let Err(e) = client.start().await.map_err(anyhow::Error::from) {
        println!("Client error: {}", e.to_string());
        return Err(e);
    }
    Ok(())
}
