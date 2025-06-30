//use csv::{Reader, StringRecord, Writer};
use crate::types::types::{Context, Data, Error};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use poise::serenity_prelude as serenity;
use serenity::model::id::ChannelId;
use serenity::prelude::*;
use anyhow::Result;
use serenity::{
    //model::prelude::{Message, Ready},
    Client,
};

use std::env;

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
async fn say(
    ctx: Context<'_>,
    #[description = "Message to say"] message: String,
) -> Result<(), Error> {
    ctx.say(message).await?;
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
                ChannelId::new(1160065321013620857), //bot
                //ChannelId::new(1120455140416172115), //genny
                env::var("LAPTOP").expect("0"),
            )
            .await?;

            // let db_location = "data/test.db";
            // let mut db = match PickleDb::load(
            //     db_location,
            //     PickleDbDumpPolicy::AutoDump,
            //     SerializationMethod::Json,
            // ) {
            //     Ok(db) => db,
            //     Err(e) => {
            //         //ctx.say("Could not load db").await?;
            //         return Err(Error::from(e.to_string()));
            //     }
            // };
            // // create a new list
            // db.lcreate("list1")?;
            // // add a bunch of numbers to the list
            // db.lextend("list1", &vec![100, 200, 300]).unwrap();
            // // get the list
            // let item: i32 = db.lget("list1", db.llen("list1") - 1).unwrap();
            // //println!("og item: {}", item);
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
                age(),
                say(),
                //commands::stars::matchup::matchup(),
                //commands::modal::modal(),
                commands::options::add::open::open(),
                commands::options::add::close::close(),
                commands::options::add::expire::expire(),
                commands::options::add::assign::assign(),
                commands::options::add::split::split(),
                commands::options::add::roll::roll(),
                commands::options::edit::edit::edit(),
                commands::options::edit::date::date(),
                commands::options::view::view::view(),
                commands::options::view::all::all(),
                commands::options::view::details::details(),
                commands::options::view::assets::assets(),
                commands::options::query::stats::stats(),
                commands::options::query::best::best(),
                commands::options::query::month::month(),
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

    if let Err(e) = client.start().await.map_err(anyhow::Error::from) {
        println!("Client error: {}", e.to_string());
        return Err(e);
    }
    Ok(())
}
