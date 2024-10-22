use chrono::prelude::*;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use tokio::sync::Mutex;

pub struct Data {
    pub db: Mutex<PickleDb>,
} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type AppContext<'a> = poise::ApplicationContext<'a, Data, Error>;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionOpen {
    pub date: DateTime<Local>,
    pub open_type: String,
    pub ticker: String,
    pub strike: f64,
    pub expiry: DateTime<Local>,
    pub premium: f64,
    pub quantity: u16,
    pub status: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionClose {
    pub date: DateTime<Local>,
    pub close_type: String,
    pub premium: f64,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Contract {
    pub open: OptionOpen,
    pub close: Option<OptionClose>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Position {
    pub contracts: Vec<Contract>,
}

pub fn open_db(path: String) -> Option<PickleDb> {
    let mut new_flag = false;
    let mut opendb = match PickleDb::load(
        path.clone(),
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    ) {
        Ok(opendb) => opendb,
        Err(e) => {
            println!("Could not load db: {}, creating new one", e.to_string());
            new_flag = true;
            PickleDb::new(
                path.clone(),
                PickleDbDumpPolicy::AutoDump,
                SerializationMethod::Json,
            )
        }
    };
    if new_flag {
        opendb.set("commission", &0.65).unwrap();
        opendb.set("edit_id", &-1).unwrap();
        opendb.lcreate("positions").unwrap();
    }
    Some(opendb)
}
