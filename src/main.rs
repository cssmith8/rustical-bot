//use csv::{Reader, StringRecord, Writer};
use crate::types::{Context, Data, Error};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use poise::serenity_prelude as serenity;
use serenity::model::id::ChannelId;
use serenity::prelude::*;
use serenity::{
    //model::prelude::{Message, Ready},
    Client,
};

use std::env;
use types::AppContext;
use utils::paginate_cool::paginate_cool;

mod commands;
mod types;
mod utils;

#[derive(Debug, serde::Deserialize)]
struct Record {
    name: String,
}

#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn say(ctx: Context<'_>, message: String) -> Result<(), Error> {
    ctx.say(message).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn paginatecool(ctx: AppContext<'_>) -> Result<(), Error> {
    let pages = &[
        "# Content of first page\n- real thing\n- fake thing",
        "Content of second page",
        "Content of third page",
        "Content of fourth page",
    ];

    paginate_cool(ctx, pages).await?;

    Ok(())
}

//send a message in channel c
async fn rustical_message(
    ctx: &serenity::Context,
    data: &Data,
    c: ChannelId,
    laptop: String,
) -> Result<(), Error> {
    let mut db = data.db.lock().await;

    let index: i32 = db.get::<i32>("line").unwrap_or_default();
    db.set("line", &(index + 1)).unwrap();

    let mut rdr = csv::Reader::from_path("./data/cool.csv")?;
    let mut results: Vec<Record> = vec![];
    for result in rdr.deserialize::<Record>() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        match result {
            Ok(rec) => results.push(rec),
            Err(err) => {
                println!("ERORR PARSING: {}", err.to_string())
            }
        }
    }

    let message = match results.get((index % (results.len() as i32)) as usize) {
        Some(res) => res.name.clone(),
        None => "Couldn't get one lol".to_string(),
    };

    let l: String = match laptop.parse().unwrap() {
        1 => " Laptopically".to_string(),
        _ => "".to_string(),
    };

    let channel = c;
    let channel = channel
        .to_channel(&ctx.http)
        .await
        .expect("this channel will always work");
    if let Some(channel) = channel.guild() {
        channel
            .say(&ctx.http, message + &l + " :money_mouth:")
            .await?;
    }
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct AmazingThing {
    real: String,
    fake: f32,
}
async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            println!("Logged in as {}", data_about_bot.user.tag());
            rustical_message(
                ctx,
                data,
                ChannelId::new(1160065321013620857),
                env::var("LAPTOP").expect("0"),
            )
            .await?;

            let mut testdb = match PickleDb::load(
                "data/test.db",
                PickleDbDumpPolicy::AutoDump,
                SerializationMethod::Json,
            ) {
                Ok(testdb) => testdb,
                Err(e) => {
                    println!("Could not load db: {}, creating new one", e.to_string());
                    PickleDb::new(
                        "data/test.db",
                        PickleDbDumpPolicy::AutoDump,
                        SerializationMethod::Json,
                    )
                }
            };

            let real_thing = AmazingThing {
                real: "real".to_string(),
                fake: 0.0,
            };
            testdb.set("real", &real_thing).unwrap();

            let index = testdb.get::<AmazingThing>("real").unwrap();
            //println!("Real: {}, Fake: {}", index.real, index.fake);
        }
        // me when the
        serenity::FullEvent::Message { new_message } => {
            if new_message.author.bot {
                return Ok(());
            }
            //not case sensitive
            if new_message.content.eq_ignore_ascii_case("rustical bot") {
                rustical_message(ctx, data, new_message.channel_id, "0".to_string()).await?;
            }
        }
        _ => {}
    };

    Ok(())
}

#[tokio::main]
async fn main() {
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
                age(),
                commands::modal::modal(),
                commands::stars::matchup::matchup(),
                say(),
                paginatecool(),
                commands::add_option_data::open(),
                commands::add_option_data::close(),
                commands::view_open::view(),
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
        //.event_handler(Handler {})
        .framework(framework)
        .await
        .expect("Could not create client");

    if let Err(e) = client.start().await {
        println!("Client error: {}", e.to_string());
    }
}
