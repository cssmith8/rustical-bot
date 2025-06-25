use crate::types::Position;
use crate::types::{AppContext, Error};
use crate::utils::{open_option_db, position_list_replace};
use chrono::prelude::*;
//use poise::serenity_prelude::CreateQuickModal;
use poise::Modal;
use anyhow::Result;

#[derive(Debug, Modal)]
#[name = "Edit Position"] // Struct name by default
struct DateModal {
    #[name = "Year"] // Field name by default
    #[placeholder = "2024"] // No placeholder by default
    #[max_length = 4]
    //#[paragraph] // Switches from single-line input to multiline text box
    year: Option<String>,
    #[name = "Month"]
    #[placeholder = "12"]
    #[max_length = 2]
    month: Option<String>,
    #[name = "Day"]
    #[placeholder = "30"]
    #[max_length = 2]
    day: Option<String>,
}

#[poise::command(slash_command)]
pub async fn date(ctx: AppContext<'_>) -> Result<(), Error> {
    let userid = ctx.interaction.user.id;
    let db_location = format!("data/options/{}.db", userid.to_string());

    let mut db = match open_option_db(db_location.clone()) {
        Some(db) => db,
        None => {
            return Err(Error::from("Could not load db"));
        }
    };
    let edit_id: i32 = match db.get("edit_id") {
        Some(id) => id,
        None => {
            return Err(Error::from("Could not retrieve edit_id"));
        }
    };
    if edit_id == -1 {
        ctx.say("No open position selected").await?;
        return Ok(());
    }
    if edit_id >= db.llen("positions") as i32 {
        ctx.say("Invalid selection").await?;
        return Ok(());
    }
    let mut position: Position = match db.lget("positions", edit_id as usize) {
        Some(pos) => pos,
        None => {
            return Err(Error::from("Could not retrieve position"));
        }
    };
    let last_index = position.contracts.len() - 1;

    let data = match DateModal::execute(ctx).await? {
        Some(data) => data,
        None => return Ok(()),
    };

    if let Some(year) = data.year {
        position.contracts[last_index].open.date = match Utc
            .with_ymd_and_hms(
                year.parse::<i32>()?,
                position.contracts[last_index].open.date.month(),
                position.contracts[last_index].open.date.day(),
                17,
                0,
                0,
            )
            {
                chrono::LocalResult::Single(datetime) => datetime,
                _ => return Err(Error::from("Invalid date provided")),
            };
    }
    if let Some(month) = data.month {
        position.contracts[last_index].open.date = match Utc
            .with_ymd_and_hms(
                position.contracts[last_index].open.date.year(),
                month.parse::<u32>()?,
                position.contracts[last_index].open.date.day(),
                17,
                0,
                0,
            )
            {
                chrono::LocalResult::Single(datetime) => datetime,
                _ => return Err(Error::from("Invalid date provided")),
            };
    }
    if let Some(day) = data.day {
        position.contracts[last_index].open.date = match Utc.with_ymd_and_hms(
            position.contracts[last_index].open.date.year(),
            position.contracts[last_index].open.date.month(),
            day.parse::<u32>()?,
            17,
            0,
            0,
        ) {
            chrono::LocalResult::Single(datetime) => datetime,
            _ => return Err(Error::from("Invalid date provided")),
        };
    }
    position_list_replace(&mut db, "positions", edit_id as usize, position);

    ctx.say("Position Updated").await?;
    db.set("edit_id", &-1)?;
    Ok(())
}
