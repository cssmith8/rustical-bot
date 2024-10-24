use crate::types::Position;
use crate::types::{AppContext, Error};
use crate::utils::{open_option_db, position_list_replace};
use chrono::prelude::*;
//use poise::serenity_prelude::CreateQuickModal;
use poise::Modal;

#[derive(Debug, Modal)]
#[name = "Edit Position"] // Struct name by default
struct EditModal {
    #[name = "Stock Ticker"] // Field name by default
    #[placeholder = "AMZN"] // No placeholder by default
    #[min_length = 2] // No length restriction by default (so, 1-4000 chars)
    #[max_length = 16]
    //#[paragraph] // Switches from single-line input to multiline text box
    ticker: Option<String>,
    #[name = "Strike Price"]
    #[placeholder = "10.00"]
    strike: Option<String>,
    #[name = "Expiration Date"]
    #[placeholder = "2024-12-30"]
    #[max_length = 10]
    exp: Option<String>,
    #[name = "Premium"]
    #[placeholder = "0.50"]
    premium: Option<String>,
    #[name = "Quantity"]
    #[placeholder = "1"]
    quantity: Option<String>,
}

#[poise::command(slash_command)]
pub async fn edit(ctx: AppContext<'_>) -> Result<(), Error> {
    let userid = ctx.interaction.user.id;
    let db_location = format!("data/options/{}.db", userid.to_string());

    let mut db = match open_option_db(db_location.clone()) {
        Some(db) => db,
        None => {
            return Err(Error::from("Could not load db"));
        }
    };
    let edit_id: i32 = db.get("edit_id").unwrap();
    if edit_id == -1 {
        ctx.say("No open position selected").await?;
        return Ok(());
    }
    if edit_id >= db.llen("positions") as i32 {
        ctx.say("Invalid selection").await?;
        return Ok(());
    }
    let mut position: Position = db.lget("positions", edit_id as usize).unwrap();
    let last_index = position.contracts.len() - 1;

    //execute the modal
    let data = match EditModal::execute(ctx).await? {
        Some(data) => data,
        None => return Ok(()),
    };

    if let Some(ticker) = data.ticker {
        position.contracts[last_index].open.ticker = ticker;
    }
    if let Some(strike) = data.strike {
        position.contracts[last_index].open.strike = strike.parse::<f64>().unwrap();
    }
    if let Some(exp) = data.exp {
        let nd = NaiveDate::parse_from_str(&exp, "%Y-%m-%d").unwrap();
        position.contracts[last_index].open.expiry = Local
            .with_ymd_and_hms(
                nd.year_ce().1 as i32,
                nd.month0() + 1,
                nd.day0() + 1,
                0,
                0,
                0,
            )
            .unwrap();
    }
    if let Some(premium) = data.premium {
        position.contracts[last_index].open.premium = premium.parse::<f64>().unwrap();
    }
    if let Some(quantity) = data.quantity {
        position.contracts[last_index].open.quantity = quantity.parse::<u16>().unwrap();
    }
    position_list_replace(&mut db, "positions", edit_id as usize, position);

    ctx.say("Position Updated").await?;
    db.set("edit_id", &-1)?;
    Ok(())
}

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
    let edit_id: i32 = db.get("edit_id").unwrap();
    if edit_id == -1 {
        ctx.say("No open position selected").await?;
        return Ok(());
    }
    if edit_id >= db.llen("positions") as i32 {
        ctx.say("Invalid selection").await?;
        return Ok(());
    }
    let mut position: Position = db.lget("positions", edit_id as usize).unwrap();
    let last_index = position.contracts.len() - 1;

    let data = match DateModal::execute(ctx).await? {
        Some(data) => data,
        None => return Ok(()),
    };

    if let Some(year) = data.year {
        position.contracts[last_index].open.date = Local
            .with_ymd_and_hms(
                year.parse::<i32>().unwrap(),
                position.contracts[last_index].open.date.month(),
                position.contracts[last_index].open.date.day(),
                0,
                0,
                0,
            )
            .unwrap();
    }
    if let Some(month) = data.month {
        position.contracts[last_index].open.date = Local
            .with_ymd_and_hms(
                position.contracts[last_index].open.date.year(),
                month.parse::<u32>().unwrap(),
                position.contracts[last_index].open.date.day(),
                0,
                0,
                0,
            )
            .unwrap();
    }
    if let Some(day) = data.day {
        position.contracts[last_index].open.date = Local
            .with_ymd_and_hms(
                position.contracts[last_index].open.date.year(),
                position.contracts[last_index].open.date.month(),
                day.parse::<u32>().unwrap(),
                0,
                0,
                0,
            )
            .unwrap();
    }
    position_list_replace(&mut db, "positions", edit_id as usize, position);

    ctx.say("Position Updated").await?;
    db.set("edit_id", &-1)?;
    Ok(())
}
