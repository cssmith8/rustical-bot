use poise::serenity_prelude::{self as serenity, Colour};
use crate::types::types::{AppContext, Error};
use crate::types::position::Position;
use crate::utils::open_option_db;

#[poise::command(slash_command)]
pub async fn stats(ctx: AppContext<'_>) -> Result<(), Error> {
    let userid = ctx.interaction.user.id;
    let db_location = format!("data/options/{}.db", userid.to_string());

    let db = match open_option_db(db_location.clone()) {
        Some(db) => db,
        None => {
            return Err(Error::from("Could not load db"));
        }
    };

    let mut positions: Vec<Position> = Vec::new();
    let mut open_positions: Vec<Position> = Vec::new();
    let mut closed_positions: Vec<Position> = Vec::new();
    for item_iter in db.liter("positions") {
        let pos = item_iter.get_item::<Position>().unwrap();
        positions.push(pos.clone());
        if pos.is_closed() {
            closed_positions.push(pos);
        } else {
            open_positions.push(pos);
        }
    }

    let gain: f64 = closed_positions.iter().map(|pos| pos.gain()).sum();
    let unrealized_gain: f64 = open_positions.iter().map(|pos| pos.gain()).sum();

    let reply = poise::CreateReply::default().embed(
        serenity::CreateEmbed::default()
            .description(format!("Total gain: `${}`\nCurrent unrealized gain: `${}`", gain, unrealized_gain))
            .color(Colour::DARK_GREEN),
    );
    ctx.send(reply).await?;
    Ok(())
}