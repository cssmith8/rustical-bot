use poise::serenity_prelude::{self as serenity, Colour};
use crate::types::positionmonth::PositionMonth;
use crate::types::tradingmonth::TradingMonth;
use crate::types::types::{AppContext, Error};
use crate::types::position::Position;
use crate::utils::open_option_db;
use std::collections::HashMap;

#[poise::command(slash_command)]
pub async fn month(ctx: AppContext<'_>) -> Result<(), Error> {
    let userid = ctx.interaction.user.id;
    let db_location = format!("data/options/{}.db", userid.to_string());

    let db = match open_option_db(db_location.clone()) {
        Some(db) => db,
        None => {
            return Err(Error::from("Could not load db"));
        }
    };

    let mut closed_positions: Vec<Position> = Vec::new();
    for item_iter in db.liter("positions") {
        let pos = item_iter.get_item::<Position>().unwrap();
        if pos.is_closed() {
            closed_positions.push(pos.clone());
        }
    }

    let mut profitmonths: Vec<PositionMonth> = Vec::new();

    for pos in &closed_positions {
        profitmonths.extend(pos.generate_profit_months());
    }

    // new hashmap that stores strings -> TradingMonth
    let mut trading_months: HashMap<String, TradingMonth> = HashMap::new();

    // for each profitmonth:
    for profitmonth in &profitmonths {
        // create a unique id for the trading month, e.g., "YYYY-MM"
        let key = profitmonth.id();

        // if there does not exist a trading month with matching id, make one in the hashmap
        trading_months
            .entry(key.clone())
            .and_modify(|tm| {
                // else use the .combine() method on the tradingmonth to add the data of the profitmonth
                tm.combine(profitmonth.clone());
            })
            .or_insert_with(|| TradingMonth {
                year: profitmonth.year,
                month: profitmonth.month,
                gain: profitmonth.gain,
                investment: profitmonth.investment
            });
    }

    let mut response = String::new();
    let mut months: Vec<&TradingMonth> = trading_months.values().collect();
    months.sort_by(|a, b| b.daily_return_rate().partial_cmp(&a.daily_return_rate()).unwrap_or(std::cmp::Ordering::Equal));
    for tradingmonth in months {
        let year = tradingmonth.year;
        let month = tradingmonth.month;
        let return_rate = tradingmonth.daily_return_rate();
        let month_name = chrono::NaiveDate::from_ymd_opt(year, month, 1)
            .map(|d| d.format("%B").to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        response.push_str(&format!("{} {}: {:.2}%\n", month_name, year, return_rate));
    }
    
    let reply = poise::CreateReply::default().embed(
        serenity::CreateEmbed::default()
            .description(format!("**Best months by daily return rate:**\n\n{}", response))
            .color(Colour::DARK_GREEN),
    );
    ctx.send(reply).await?;
    Ok(())
}