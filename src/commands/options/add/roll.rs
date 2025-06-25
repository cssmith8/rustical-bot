use crate::types::{AppContext, Error};
use crate::types::{Contract, OptionClose, OptionOpen, Position};
use crate::utils::{open_option_db, position_list_replace};
use chrono::prelude::*;
use anyhow::Result;
use poise::Modal;

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
    let expiry = match Utc.with_ymd_and_hms(
        nd.year_ce().1 as i32,
        nd.month0() + 1,
        nd.day0() + 1,
        20,
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
        date: Utc::now(),
        close_type: "roll".to_string(),
        premium: data.premium_loss.parse::<f64>()?,
    });

    let strike = match data.strike {
        Some(s) => s.parse::<f64>()?,
        None => position.contracts[last_index].open.strike,
    };

    position.contracts.push(Contract {
        open: OptionOpen {
            date: Utc::now(),
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
