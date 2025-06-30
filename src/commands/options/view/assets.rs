use crate::types::types::{AppContext, Error};
use crate::types::position::Position;
use poise::serenity_prelude::{self as serenity, Colour};
use crate::utils::open_option_db;
use std::collections::HashMap;

#[poise::command(slash_command)]
pub async fn assets(ctx: AppContext<'_>) -> Result<(), Error> {
    let userid = ctx.interaction.user.id;
    let db_location = format!("data/options/{}.db", userid.to_string());

    let db = match open_option_db(db_location.clone()) {
        Some(db) => db,
        None => {
            return Err(Error::from("Could not load db"));
        }
    };

    // Create empty hash map of String tickers to int quantities
    let mut ticker_quantities: HashMap<String, i32> = HashMap::new();

    for item_iter in db.liter("positions") {
        let pos: Position = item_iter.get_item::<Position>().unwrap();

        // Ignore if the status is "open"
        if pos.get_status() == "open" {
            continue;
        }

        // Determine the multiplier based on position type
        let qty = 100 * pos.get_final_contract().quantity() as i32;
        let entry = ticker_quantities.entry(pos.get_ticker().clone()).or_insert(0);

        match pos.option_type().as_str() {
            "put" => *entry += qty,
            "call" => *entry -= qty,
            _ => {}
        }
    }

    // Remove entries from the hashmap that have zero quantity
    ticker_quantities.retain(|_, &mut qty| qty != 0);

    let mut response = "".to_string();
    // For each entry in the hash map, append the ticker and quantity to the response string
    for (ticker, qty) in &ticker_quantities {
        response.push_str(&format!("`{} {}`\n", qty, ticker));
    }

    let reply = poise::CreateReply::default().embed(
        serenity::CreateEmbed::default()
            .description(format!("Your current assets: \n{}", response))
            .color(Colour::DARK_GREEN),
    );
    ctx.send(reply).await?;
    Ok(())
}