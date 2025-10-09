use crate::events::handler::event_handler;
use crate::types::types::Data;
use crate::utils::db::create_or_open_db;
use crate::utils::env;
use anyhow::Result;
use poise::serenity_prelude as serenity;
use serenity::prelude::*;
use serenity::Client;

mod commands;
mod events;
mod types;
mod utils;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    let token = env::discord_token();

    let db = create_or_open_db(env::data_path() + "real.db");

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                // commands::say::say(),
                // commands::joke::joke(),
                // commands::remark::remark(),
                // commands::translate::translate(),
                // commands::logs::logs(),
                // commands::realtime::realtime(),
                commands::clear::clear(),
                commands::stratagem::stratagem()
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
                    _db: Mutex::new(db),
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
