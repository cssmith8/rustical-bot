use crate::commands::option_settings::{get_setting, edit_settings};
use crate::types::{AppContext, Error};
use crate::types::{OptionClose, OptionOpen, OptionAssignment};
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
pub async fn open(ctx: AppContext<'_>, #[choices("put","call")] open_type: &'static str) -> Result<(), Error> {
    //if the open_type is not put or call, return
    if open_type != "put" && open_type != "call" {
        ctx.say("Invalid option type").await?;
        return Ok(());
    }
    // todo: viewopen option type display
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
                open_type.to_string(),
            )
            .await?;
            ctx.say("Contract Opened").await?;

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
    open_type: String,
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
        open_type,
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

    return Ok(end_id);
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
    let data = CloseModal::execute(ctx).await?;

    match data {
        Some(data) => {
            //get current date
            let date = Local::now();
            //parse the premium
            let premium = data.premium.parse::<f64>().unwrap();
            //parse the quantity
            //let quantity = data.quantity.parse::<u16>().unwrap();
            let open_id = edit_id.parse::<u32>().unwrap();
            //add the close contract to the database
            let close_id = add_close(userid, date, "close".to_string(), open_id, -1, premium, 0).await?;
            open.close_id = Some(close_id);
            open.status = "closed".to_string();
            opendb.set(edit_id.as_str(), &open).unwrap();
            let gain: f64 = (open.premium - premium) * (open.quantity as f64) * 100 as f64;
            let money_mouth = if gain > 0.0 {
                ":money_mouth:"
            } else {
                ""
            };
            ctx.say(format!("Contract Closed. You made ${} {}", gain, money_mouth)).await?;
        }
        None => return Ok(()),
    }
    edit_settings(userid, "edit_id".to_string(), "-1".to_string()).await?;
    Ok(())
}

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

    ctx.say(format!("Assigned {} shares of {}", open.quantity * 100, open.ticker)).await?;

    edit_settings(userid, "edit_id".to_string(), "-1".to_string()).await?;
    Ok(())
}

#[allow(dead_code)]
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

    return Ok(end_id);
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