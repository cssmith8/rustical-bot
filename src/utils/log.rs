use crate::{
    types::{dblog::DBLog, types::Error},
    utils::db::create_or_open_db,
};
use chrono::Utc;
use poise::serenity_prelude::{self as serenity, Http};
use serenity::model::id::ChannelId;
use std::env;

#[allow(dead_code)]
pub fn log(message: String) {
    let mut db = create_or_open_db(format!(
        "{}/logs.db",
        env::var("DB_PATH").unwrap_or_else(|_| "data/".into())
    ));
    if !db.lexists("logs") {
        if db.lcreate("logs").is_err() {
            return;
        }
    }
    if db
        .ladd(
            "logs",
            &DBLog {
                timestamp: Utc::now(),
                message: message.clone(),
            },
        )
        .is_none()
    {
        return;
    }
    if db.get::<bool>("realtime").unwrap_or(false) {
        send_realtime_log(&message);
    }
    println!("[Log]: {}", message);
}

fn send_realtime_log(message: &str) {
    let channel = ChannelId::new(1160065321013620857);
    let http = Http::new(&env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN must be set"));

    // Spawn a new Tokio task to send the message asynchronously
    let message = message.to_string();
    tokio::spawn(async move {
        let _ = channel.say(&http, format!("[Log]: {}", message)).await;
    });
}

pub fn load_all_logs() -> Result<Vec<DBLog>, Error> {
    let db = create_or_open_db("data/logs.db".to_string());

    let mut all_logs: Vec<DBLog> = Vec::new();
    for item_iter in db.liter("logs") {
        let db_log = item_iter.get_item::<DBLog>().unwrap();
        all_logs.push(db_log);
    }
    Ok(all_logs)
}
