use crate::types::{AppContext, Error};
use crate::types::{OptionAssignment, OptionClose, OptionOpen};
use chrono::prelude::*;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use poise::{serenity_prelude::model::id, Modal};

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
pub async fn open(ctx: AppContext<'_>) -> Result<(), Error> {
    let data = OpenModal::execute(ctx).await?;
    //println!("Got data: {:?}", data);
    match data {
        Some(data) => {
            //get user id
            let userid = ctx.interaction.user.id;
            //get current date
            let date = Local::now();
            //parse the strike price
            let strike = data.strike.parse::<f64>().unwrap();
            //parse the expiration date
            let nd = NaiveDate::parse_from_str(&data.exp, "%Y-%m-%d").unwrap();
            let exp = Local
                .with_ymd_and_hms(
                    nd.year_ce().1 as i32,
                    nd.month0() + 1,
                    nd.day0() + 1,
                    0,
                    0,
                    0,
                )
                .unwrap();
            //parse the premium
            let premium = data.premium.parse::<f64>().unwrap();
            //parse the quantity
            let quantity = data.quantity.parse::<u16>().unwrap();
            //get the status
            let status = "open".to_string();
            //get the close id
            let close_id: Option<u32> = None;
            //add the open contract to the database
            add_open(
                userid,
                date,
                data.ticker,
                strike,
                exp,
                premium,
                quantity,
                status,
                close_id,
            )
            .await?;
            ctx.say("real").await?;
        }
        None => return Ok(()),
    }
    Ok(())
}

pub async fn add_open(
    userid: id::UserId,
    date: DateTime<Local>,
    ticker: String,
    strike: f64,
    expiry: DateTime<Local>,
    premium: f64,
    quantity: u16,
    status: String,
    close_id: Option<u32>,
) -> Result<u32, Error> {
    let db_location = format!("data/{}_open.db", userid.to_string());
    let mut new_flag = false;
    let mut opendb = match PickleDb::load(
        db_location.clone(),
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    ) {
        Ok(opendb) => opendb,
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
        opendb.set("end_id", &0).unwrap();
    }
    let end_id = opendb.get::<u32>("end_id").unwrap();
    let new_open = OptionOpen {
        id: end_id,
        date,
        ticker,
        strike,
        expiry,
        premium,
        quantity,
        status,
        close_id,
    };
    opendb.set(end_id.to_string().as_str(), &new_open).unwrap();
    opendb.set("end_id", &(end_id + 1)).unwrap();

    return Ok(0);
}

pub async fn add_close(
    userid: id::UserId,
    date: DateTime<Local>,
    close_type: String,
    open_id: u32,
    roll_id: u32,
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

    return Ok(0);
}

pub async fn add_assignment(
    userid: id::UserId,
    date: DateTime<Local>,
    open_id: u32,
    ticker: String,
    strike: f64,
    quantity: u16,
) -> Result<u32, Error> {
    let db_location = format!("data/{}_assignment.db", userid.to_string());
    let mut new_flag = false;
    let mut assignmentdb = match PickleDb::load(
        db_location.clone(),
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    ) {
        Ok(assignmentdb) => assignmentdb,
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
        assignmentdb.set("end_id", &0).unwrap();
    }
    let end_id = assignmentdb.get::<u32>("end_id").unwrap();
    let new_assignment = OptionAssignment {
        id: end_id,
        date,
        open_id,
        ticker,
        strike,
        quantity,
    };
    assignmentdb
        .set(end_id.to_string().as_str(), &new_assignment)
        .unwrap();
    assignmentdb.set("end_id", &(end_id + 1)).unwrap();

    return Ok(0);
}
