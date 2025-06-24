use crate::types::{AppContext, Error, Position};
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
    for item_iter in db.liter("positions") {
        positions.push(item_iter.get_item::<Position>().unwrap());
    }

    ctx.say(format!("Total investment time: {}", 0)).await?;
    Ok(())
}