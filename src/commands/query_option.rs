use crate::types::{AppContext, Error, Position};
use crate::utils::open_option_db;
use chrono::Datelike;

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

#[poise::command(slash_command)]
pub async fn best(ctx: AppContext<'_>) -> Result<(), Error> {
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
        let position = item_iter.get_item::<Position>().unwrap();
        if position.is_closed() {
            positions.push(position);
        }
    }

    positions.sort_by(|a, b| {
        let roi_a = a.return_on_investment() / a.time() as f64;
        let roi_b = b.return_on_investment() / b.time() as f64;
        roi_b.partial_cmp(&roi_a).unwrap()
    });

    let best_positions: Vec<String> = positions.iter().take(3).map(|p| {
        let ticker = p.contracts[0].open.ticker.clone();
        let open_date = format!("{}/{}/{}", p.contracts[0].open.date.month(), p.contracts[0].open.date.day(), p.contracts[0].open.date.year() % 100);
        let gain = p.gain();
        let investment = p.investment();
        let duration = p.time();
        let duration_plural = if duration > 1 { "s" } else { "" };
        format!(
            "```\n{} - {}\nGained ${:.2} from investment of ${:.2} over {} day{}\nDaily ROI: {:.2}%```",
            open_date, ticker, gain, investment, duration, duration_plural, p.return_on_investment() * 100.0 / duration as f64
        )
    }).collect();

    ctx.say(format!("Top 3 positions:\n{}", best_positions.join(""))).await?;
    Ok(())
}