use crate::{
    types::types::{AppContext, Error},
    utils::db::create_or_open_db,
};
use std::env;

/// Enable or disable realtime logging
#[poise::command(slash_command)]
pub async fn realtime(
    ctx: AppContext<'_>,
    #[choices("on", "off")] value: &'static str,
) -> Result<(), Error> {
    let mut db = create_or_open_db(format!(
        "{}/logs.db",
        env::var("DB_PATH").unwrap_or_else(|_| "data/".into())
    ));
    db.set("realtime", &(value == "on"))?;
    let status = if value == "on" { "enabled" } else { "disabled" };
    ctx.say(format!("Realtime logging {}", status)).await?;
    Ok(())
}
