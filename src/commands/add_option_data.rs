use crate::types::{AppContext, Error};
use crate::types::{Contract, OptionClose, OptionOpen, Position};
use crate::utils::{open_option_db, position_list_replace};
use chrono::prelude::*;
use anyhow::Result;
use poise::Modal;

#[derive(Debug, Modal)]
#[name = "Open Contract"] // Struct name by default
pub struct OpenModal {
    #[name = "Stock Ticker"] // Field name by default
    #[placeholder = "AMZN"] // No placeholder by default
    #[min_length = 1] // No length restriction by default (so, 1-4000 chars)
    #[max_length = 16]
    //#[paragraph] // Switches from single-line input to multiline text box
    ticker: String,
    #[name = "Strike Price"]
    #[placeholder = "10.00"]
    strike: String,
    #[name = "Expiration Date"]
    #[placeholder = "2024-12-30"]
    #[max_length = 10]
    exp: String,
    #[name = "Premium"]
    #[placeholder = "0.50"]
    premium: String,
    #[name = "Quantity"]
    #[placeholder = "1"]
    quantity: String,
}

#[poise::command(slash_command)]
pub async fn open(
    ctx: AppContext<'_>,
    #[choices("put", "call")] open_type: &'static str,
) -> Result<(), Error> {
    //if the open_type is not put or call, return
    if open_type != "put" && open_type != "call" {
        ctx.say("Invalid option type").await?;
        return Ok(());
    }
    let userid = ctx.interaction.user.id;
    let data = OpenModal::execute(ctx).await?;
    match data {
        Some(data) => {
            //get modal info
            let date = Local::now();
            let strike = data.strike.parse::<f64>()?;
            let nd = NaiveDate::parse_from_str(&data.exp, "%Y-%m-%d")?;
            let expiry = match Local.with_ymd_and_hms(
                nd.year_ce().1 as i32,
                nd.month0() + 1,
                nd.day0() + 1,
                0,
                0,
                0,
            ) {
                chrono::LocalResult::Single(datetime) => datetime,
                _ => return Err(Error::from("Invalid date")),
            };
            let premium = data.premium.parse::<f64>()?;
            let quantity = data.quantity.parse::<u16>()?;

            let status = "open".to_string();
            //add the open contract to the database
            let db_location = format!("data/options/{}.db", userid.to_string());
            let mut db = match open_option_db(db_location.clone()) {
                Some(db) => db,
                None => {
                    return Err(Error::from("Could not load db"));
                }
            };
            db.ladd(
                "positions",
                &Position {
                    contracts: vec![Contract {
                        open: OptionOpen {
                            date,
                            open_type: open_type.to_string(),
                            ticker: data.ticker,
                            strike,
                            expiry,
                            premium,
                            quantity,
                            status,
                        },
                        close: None,
                    }],
                },
            )
            .ok_or_else(|| Error::from("Failed to add position to database"))?;
            ctx.say("Contract Opened").await?;
        }
        None => return Ok(()),
    }
    Ok(())
}

#[derive(Debug, Modal)]
#[name = "Close Contract"] // Struct name by default
pub struct CloseModal {
    #[name = "Price"]
    #[placeholder = "0.10"]
    premium: String,
    //#[name = "Quantity"]
    //#[placeholder = "1"]
    //quantity: String,
}

#[poise::command(slash_command)]
pub async fn close(ctx: AppContext<'_>) -> Result<(), Error> {
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
            ctx.say("Failed to retrieve edit_id").await?;
            return Ok(());
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
    //execute the modal
    let data = match CloseModal::execute(ctx).await? {
        Some(data) => data,
        None => return Ok(()),
    };
    //get the position at index edit_id
    let mut position: Position = match db.lget("positions", edit_id as usize) {
        Some(pos) => pos,
        None => {
            ctx.say("Failed to retrieve position").await?;
            return Ok(());
        }
    };
    let last_index = position.contracts.len() - 1;
    position.contracts[last_index].close = Some(OptionClose {
        date: Local::now(),
        close_type: "close".to_string(),
        premium: data.premium.parse::<f64>()?,
    });
    position.contracts[last_index].open.status = "closed".to_string();
    let gain: f64 = (position.contracts[last_index].open.premium
        - data.premium.parse::<f64>()?)
        * (position.contracts[last_index].open.quantity as f64)
        * 100 as f64;
    position_list_replace(&mut db, "positions", edit_id as usize, position);
    let money_mouth = if gain > 0.0 { ":money_mouth:" } else { "" };
    ctx.say(format!(
        "Contract Closed. You made ${} {}",
        gain, money_mouth
    ))
    .await?;
    db.set("edit_id", &-1)?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn assign(ctx: AppContext<'_>) -> Result<(), Error> {
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
            ctx.say("Failed to retrieve edit_id").await?;
            return Ok(());
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
    //get the position at index edit_id
    let mut position: Position = match db.lget("positions", edit_id as usize) {
        Some(pos) => pos,
        None => {
            ctx.say("Failed to retrieve position").await?;
            return Ok(());
        }
    };
    let last_index = position.contracts.len() - 1;
    position.contracts[last_index].open.status = "assigned".to_string();
    let q = position.contracts[last_index].open.quantity;
    let ticker = position.contracts[last_index].open.ticker.clone();
    position_list_replace(&mut db, "positions", edit_id as usize, position);

    ctx.say(format!("Assigned {} shares of {}", q * 100, ticker))
        .await?;

    db.set("edit_id", &-1)?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn expire(ctx: AppContext<'_>) -> Result<(), Error> {
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
            ctx.say("Failed to retrieve edit_id").await?;
            return Ok(());
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
    //get the position at index edit_id
    let mut position: Position = match db.lget("positions", edit_id as usize) {
        Some(pos) => pos,
        None => {
            ctx.say("Failed to retrieve position").await?;
            return Ok(());
        }
    };
    let last_index = position.contracts.len() - 1;
    position.contracts[last_index].open.status = "expired".to_string();
    position_list_replace(&mut db, "positions", edit_id as usize, position);

    ctx.say("Contract Expired :money_mouth:").await?;

    db.set("edit_id", &-1)?;
    Ok(())
}

#[derive(Debug, Modal)]
#[name = "Roll Contract"] // Struct name by default
pub struct RollModal {
    #[name = "New Expiration Date"]
    #[placeholder = "2024-12-30"]
    #[max_length = 10]
    exp: String,
    #[name = "Premium Loss"]
    #[placeholder = "0.80"]
    premium_loss: String,
    #[name = "Premium Gain"]
    #[placeholder = "0.85"]
    premium_gain: String,
    #[name = "New Strike Price (Leave blank if unchanged)"]
    #[placeholder = "15"]
    strike: Option<String>,
}

#[poise::command(slash_command)]
pub async fn roll(ctx: AppContext<'_>) -> Result<(), Error> {
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
            ctx.say("Failed to retrieve edit_id").await?;
            return Ok(());
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
            ctx.say("Failed to retrieve position").await?;
            return Ok(());
        }
    };

    let data = match RollModal::execute(ctx).await? {
        Some(data) => data,
        None => return Ok(()),
    };

    let nd = NaiveDate::parse_from_str(&data.exp, "%Y-%m-%d")?;
    let expiry = match Local.with_ymd_and_hms(
        nd.year_ce().1 as i32,
        nd.month0() + 1,
        nd.day0() + 1,
        0,
        0,
        0,
    ) {
        chrono::LocalResult::Single(datetime) => datetime,
        _ => return Err(Error::from("Invalid date")),
    };
    let premium_gain = data.premium_gain.parse::<f64>()?;

    let last_index = position.contracts.len() - 1;
    position.contracts[last_index].open.status = "rolled".to_string();
    position.contracts[last_index].close = Some(OptionClose {
        date: Local::now(),
        close_type: "roll".to_string(),
        premium: data.premium_loss.parse::<f64>()?,
    });

    let strike = match data.strike {
        Some(s) => s.parse::<f64>()?,
        None => position.contracts[last_index].open.strike,
    };

    position.contracts.push(Contract {
        open: OptionOpen {
            date: Local::now(),
            open_type: position.contracts[last_index].open.open_type.clone(),
            ticker: position.contracts[last_index].open.ticker.clone(),
            strike: strike,
            expiry,
            premium: premium_gain,
            quantity: position.contracts[last_index].open.quantity,
            status: "open".to_string(),
        },
        close: None,
    });
    position_list_replace(&mut db, "positions", edit_id as usize, position);
    ctx.say("Contract Rolled").await?;
    Ok(())
}
