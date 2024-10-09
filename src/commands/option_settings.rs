use crate::types::Error;
use poise::serenity_prelude::model::id;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

pub async fn edit_settings(userid: id::UserId, key: String, value: String) -> Result<(), Error> {
    //get the user id
    let db_location = format!("data/{}.db", userid.to_string());
    let mut new_flag = false;
    let mut db = match PickleDb::load(
        db_location.clone(),
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    ) {
        Ok(db) => db,
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
        db.set("commission", &0.65).unwrap();
        db.set("edit_id", &-1).unwrap();
    }
    Ok(())
}

//get setting
pub async fn get_setting(userid: id::UserId, key: String) -> Result<String, Error> {
    //get the user id
    let db_location = format!("data/{}.db", userid.to_string());
    let mut new_flag = false;
    let mut db = match PickleDb::load(
        db_location.clone(),
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    ) {
        Ok(db) => db,
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
        db.set("commission", &0.65).unwrap();
        db.set("edit_id", &-1).unwrap();
    }
    Ok(db.get(key.as_str()).unwrap())
}
