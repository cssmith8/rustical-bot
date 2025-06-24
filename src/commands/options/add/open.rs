use crate::types::{AppContext, Error};
use crate::types::{Contract, OptionOpen, Position};
use crate::utils::{open_option_db};
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