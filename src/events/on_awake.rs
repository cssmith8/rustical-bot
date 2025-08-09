use crate::types::types::{Data, Error};
use anyhow::Result;
use poise::serenity_prelude as serenity;
use serenity::model::id::ChannelId;

use std::env;

pub async fn on_awake(
    _ctx: &serenity::Context,
    _event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
    data_about_bot: &serenity::Ready,
) -> Result<(), Error> {
    println!("Logged in as {}", data_about_bot.user.tag());

    rustical_message(
        _ctx,
        _data,
        ChannelId::new(1160065321013620857), //bot
        //ChannelId::new(1120455140416172115), //genny
        env::var("LAPTOP").expect("0"),
    )
    .await?;

    Ok(())
}

#[derive(Debug, serde::Deserialize)]
struct Record {
    name: String,
}

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
