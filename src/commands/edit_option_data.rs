use crate::commands::option_settings::{get_setting, edit_settings};
use crate::types::{AppContext, Error};
use crate::types::OptionOpen;
use chrono::prelude::*;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use poise::Modal;

#[derive(Debug, Modal)]
    #[name = "Edit Position"] // Struct name by default
    struct EditModal {
        #[name = "Stock Ticker"] // Field name by default
        #[placeholder = "AMZN"] // No placeholder by default
        #[min_length = 2] // No length restriction by default (so, 1-4000 chars)
        #[max_length = 16]
        //#[paragraph] // Switches from single-line input to multiline text box
        ticker: Option<String>,
        #[name = "Strike Price"]
        #[placeholder = "10.00"]
        strike: Option<String>,
        #[name = "Expiration Date"]
        #[placeholder = "2024-12-30"]
        #[max_length = 10]
        exp: Option<String>,
        #[name = "Premium"]
        #[placeholder = "0.50"]
        premium: Option<String>,
        #[name = "Quantity"]
        #[placeholder = "1"]
        quantity: Option<String>,
    }

#[poise::command(slash_command)]
pub async fn edit(ctx: AppContext<'_>) -> Result<(), Error> {
    let userid = ctx.interaction.user.id;

    let edit_id = get_setting(userid, "edit_id".to_string()).await?;
    if edit_id == "-1" {
        ctx.say("No open position selected").await?;
        return Ok(());
    }
    let db_location = format!("data/{}_open.db", userid.to_string());
    let mut opendb = match PickleDb::load(
        db_location.clone(),
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    ) {
        Ok(opendb) => opendb,
        Err(e) => {
            ctx.say("Could not load db").await?;
            return Err(Error::from(e.to_string()));
        }
    };
    let mut open = opendb.get::<OptionOpen>(edit_id.as_str()).unwrap();

    let data = EditModal::execute(ctx).await?;
    match data {
        Some(data) => {
            if let Some(ticker) = data.ticker {
                open.ticker = ticker;
            }
            if let Some(strike) = data.strike {
                open.strike = strike.parse::<f64>().unwrap();
            }
            if let Some(exp) = data.exp {
                let nd = NaiveDate::parse_from_str(&exp, "%Y-%m-%d").unwrap();
                open.expiry = Local
                    .with_ymd_and_hms(
                        nd.year_ce().1 as i32,
                        nd.month0() + 1,
                        nd.day0() + 1,
                        0,
                        0,
                        0,
                    )
                    .unwrap();
            }
            if let Some(premium) = data.premium {
                open.premium = premium.parse::<f64>().unwrap();
            }
            if let Some(quantity) = data.quantity {
                open.quantity = quantity.parse::<u16>().unwrap();
            }
            //add the open contract to the database
            opendb.set(edit_id.as_str(), &open).unwrap();
            ctx.say("Position Updated").await?;

        }
        None => return Ok(()),
    }
    edit_settings(userid, "edit_id".to_string(), "-1".to_string()).await?;
    Ok(())
}