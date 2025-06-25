use crate::types::{AppContext, Error};
use crate::types::{OptionClose, Position};
use crate::utils::{open_option_db, position_list_replace};
use chrono::prelude::*;
use anyhow::Result;
use poise::Modal;

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
        date: Utc::now(),
        close_type: "close".to_string(),
        premium: data.premium.parse::<f64>()?,
    });
    position.contracts[last_index].open.status = "closed".to_string();
    let gain: f64 = position.gain();
    position_list_replace(&mut db, "positions", edit_id as usize, position);
    let money_mouth = if gain > 0.0 { ":money_mouth:" } else { "" };
    ctx.say(format!(
        "Contract Closed. You made ${:.2} {}",
        gain, money_mouth
    ))
    .await?;
    db.set("edit_id", &-1)?;
    Ok(())
}