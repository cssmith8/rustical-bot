use crate::types::types::{AppContext, Error};
use crate::types::position::Position;
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

    let data = match DateModal::execute(ctx).await? {
        Some(data) => data,
        None => return Ok(()),
    };

    // Extract current date fields before mutably borrowing contracts
    let (mut cur_year, mut cur_month, mut cur_day) = {
        let final_date = position.get_final_contract().open.date;
        (final_date.year(), final_date.month(), final_date.day())
    };

    let last_idx = position.contracts.len() - 1;

    // Track if any fields were updated
    let mut updated_fields = false;

    // Update the working values instead of the position directly
    if let Some(year) = data.year {
        cur_year = year.parse::<i32>()?;
        updated_fields = true;
    }
    
    if let Some(month) = data.month {
        cur_month = month.parse::<u32>()?;
        updated_fields = true;
    }
    
    if let Some(day) = data.day {
        cur_day = day.parse::<u32>()?;
        updated_fields = true;
    }

    // Apply all changes at once if any fields were updated
    if updated_fields {
        position.contracts[last_idx].open.date = match Utc.with_ymd_and_hms(
            cur_year,
            cur_month,
            cur_day,
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
