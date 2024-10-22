use crate::commands::option_settings::{edit_settings, get_setting};
use crate::types::{position_list_replace, AppContext, Error};
use crate::types::{Contract, OptionClose, OptionOpen, Position};
use crate::types::open_option_db;
use chrono::prelude::*;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use poise::Modal;

#[derive(Debug, Modal)]
#[name = "Open Contract"] // Struct name by default
pub struct OpenModal {
    #[name = "Stock Ticker"] // Field name by default
    #[placeholder = "AMZN"] // No placeholder by default
    #[min_length = 2] // No length restriction by default (so, 1-4000 chars)
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
            let strike = data.strike.parse::<f64>().unwrap();
            let nd = NaiveDate::parse_from_str(&data.exp, "%Y-%m-%d").unwrap();
            let expiry = Local
                .with_ymd_and_hms(
                    nd.year_ce().1 as i32,
                    nd.month0() + 1,
                    nd.day0() + 1,
                    0,
                    0,
                    0,
                )
                .unwrap();
            let premium = data.premium.parse::<f64>().unwrap();
            let quantity = data.quantity.parse::<u16>().unwrap();

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
            .unwrap();
            ctx.say("Contract Opened").await?;
        }
        None => return Ok(()),
    }
    Ok(())
}

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
    let edit_id: i32 = db.get("edit_id").unwrap();
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
    let mut position: Position = db.lget("positions", edit_id as usize).unwrap();
    let last_index = position.contracts.len() - 1;
    position.contracts[last_index].close = Some(OptionClose {
        date: Local::now(),
        close_type: "close".to_string(),
        premium: data.premium.parse::<f64>().unwrap(),
    });
    position.contracts[last_index].open.status = "closed".to_string();
    position_list_replace(&mut db, "positions", edit_id as usize, position);
    // let gain: f64 = (open.premium - premium) * (open.quantity as f64) * 100 as f64;
    // let money_mouth = if gain > 0.0 { ":money_mouth:" } else { "" };
    // ctx.say(format!(
    //     "Contract Closed. You made ${} {}",
    //     gain, money_mouth
    // ))
    // .await?;
    // edit_settings(userid, "edit_id".to_string(), "-1".to_string()).await?;
    Ok(())
}
/*
pub async fn add_close(
    userid: id::UserId,
    date: DateTime<Local>,
    close_type: String,
    open_id: u32,
    roll_id: i32,
    premium: f64,
    quantity: u16,
) -> Result<u32, Error> {
    let db_location = format!("data/{}_close.db", userid.to_string());
    let mut new_flag = false;
    let mut closedb = match PickleDb::load(
        db_location.clone(),
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    ) {
        Ok(closedb) => closedb,
        Err(e) => {
            println!("Could not load db: {}, creating new one", e.to_string());
            new_flag = true;
            PickleDb::new(
                db_location.clone(),
                PickleDbDumpPolicy::AutoDump,
                SerializationMethod::Json,
            )
        }
    };
    if new_flag {
        closedb.set("end_id", &0).unwrap();
    }
    let end_id = closedb.get::<u32>("end_id").unwrap();
    let new_close = OptionClose {
        id: end_id,
        date,
        close_type,
        open_id,
        roll_id,
        premium,
        quantity,
    };
    closedb
        .set(end_id.to_string().as_str(), &new_close)
        .unwrap();
    closedb.set("end_id", &(end_id + 1)).unwrap();

    return Ok(end_id);
}
    */

#[poise::command(slash_command)]
pub async fn assign(ctx: AppContext<'_>) -> Result<(), Error> {
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

    //let open_id = edit_id.parse::<u32>().unwrap();
    //let assign_id = add_assignment(ctx.interaction.user.id, open.date, open_id, open.ticker.clone(), open.strike, open.quantity).await?;

    open.status = "assigned".to_string();
    //open.close_id = Some(assign_id);
    opendb.set(edit_id.as_str(), &open).unwrap();

    ctx.say(format!(
        "Assigned {} shares of {}",
        open.quantity * 100,
        open.ticker
    ))
    .await?;

    edit_settings(userid, "edit_id".to_string(), "-1".to_string()).await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn expire(ctx: AppContext<'_>) -> Result<(), Error> {
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
    open.status = "expired".to_string();
    opendb.set(edit_id.as_str(), &open).unwrap();

    ctx.say("Position Expired :money_mouth:").await?;

    edit_settings(userid, "edit_id".to_string(), "-1".to_string()).await?;
    Ok(())
}
