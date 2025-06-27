use crate::types::types::{AppContext, Error};
use crate::types::position::Position;
use crate::utils::{open_option_db, position_list_replace};
use anyhow::Result;
use poise::Modal;
    
#[derive(Debug, Modal)]
#[name = "Split Contract"] // Struct name by default
pub struct SplitModal {
    #[name = "Split Quantity"]
    #[placeholder = "1"]
    quantity: String,
}

//command that splits an existing position into 2 copies, each with less quantity
#[poise::command(slash_command)]
pub async fn split(ctx: AppContext<'_>) -> Result<(), Error> {
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
    let original_quantity = position.contracts[last_index].open.quantity;

    // Execute the modal to get split quantity
    let data = match SplitModal::execute(ctx).await? {
        Some(data) => data,
        None => return Ok(()),
    };

    let split_quantity = data.quantity.parse::<u16>()?;

    //make sure the provided quantity is between 0 and the original quantity, exclusive
    if split_quantity == 0 || split_quantity >= original_quantity {
        ctx.say("Split quantity must be greater than 0 and less than the original quantity").await?;
        return Ok(());
    }

    // Adjust the quantity of all contracts in the selected position to subtract the given amount
    for contract in &mut position.contracts {
        contract.open.quantity = contract.open.quantity.saturating_sub(split_quantity);
    }

    // Create a duplicate of the original position
    let mut duplicate_position = position.clone();

    // Set the quantity of all contracts in the duplicate to the split amount
    for contract in &mut duplicate_position.contracts {
        contract.open.quantity = split_quantity;
    }

    //save the updated original position
    position_list_replace(&mut db, "positions", edit_id as usize, position);

    //save the new position
    db.ladd("positions", &duplicate_position)
        .ok_or_else(|| Error::from("Failed to add split position to database"))?;

    ctx.say(&format!("Position split successfully. Original quantity: {}, Split quantity: {}", 
                     original_quantity - split_quantity, split_quantity)).await?;

    Ok(())
}
