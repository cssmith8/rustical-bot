use crate::events::handler::event_handler;
use crate::types::types::Data;
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
                commands::joke::joke(),
                commands::remark::remark(),
                commands::translate::translate(),
                commands::logs::logs(),
                commands::realtime::realtime(),
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
