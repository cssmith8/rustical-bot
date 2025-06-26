use poise::serenity_prelude::{self as serenity, Colour};
use crate::types::{AppContext, Error, Position};
use crate::utils::open_option_db;
use chrono::{Datelike, Duration, NaiveDate};
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

    let mut month_percents: HashMap<String, f64> = HashMap::new();

    for pos in &closed_positions {
        let return_rate = pos.return_on_investment();

        let startdate = pos.get_first_contract().open.date;
        let enddate = if pos.get_final_contract().close.is_some() {
            pos.get_final_contract().close.as_ref().unwrap().date
        } else {
            pos.get_final_contract().expiry()
        };

        let mut current = startdate.naive_utc().date();
        let enddate_naive_dt = enddate.naive_utc();
        let enddate_naive = enddate_naive_dt.date();
        let total_days = (enddate_naive - current).num_days() + 1;

        while current <= enddate_naive {
            let year = current.year();
            let month = current.month();
            let month_start = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
            let next_month = if month == 12 {
                NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
            } else {
                NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
            };
            let month_end = (next_month - Duration::days(1)).min(enddate_naive);

            let range_start = current.max(month_start);
            let range_end = month_end.min(enddate_naive);
            let days_in_month = (range_end - range_start).num_days() + 1;

            let key = format!("{},{}", year, month);
            let percent = days_in_month as f64 / total_days as f64;
            *month_percents.entry(key).or_insert(0.0) += percent * return_rate * 100.0;

            current = next_month;
        }
    }
    let mut response = String::new();
    let mut month_percent_vec: Vec<_> = month_percents.into_iter().collect();
    month_percent_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    for (key, percent) in month_percent_vec {
        let parts: Vec<&str> = key.split(',').collect();
        let year: i32 = parts[0].parse().unwrap_or(0);
        let month: u32 = parts[1].parse().unwrap_or(1);
        let month_name = chrono::NaiveDate::from_ymd_opt(year, month, 1)
            .map(|d| d.format("%B").to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        response.push_str(&format!("{} {}: {:.2}%\n", month_name, year, percent));
    }
    
    let reply = poise::CreateReply::default().embed(
        serenity::CreateEmbed::default()
            .description(format!("{}", response))
            .color(Colour::DARK_GREEN),
    );
    ctx.send(reply).await?;
    Ok(())
}