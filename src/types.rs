use chrono::prelude::*;
use pickledb::PickleDb;
use tokio::sync::Mutex;

pub struct Data {
    pub db: Mutex<PickleDb>,
} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type AppContext<'a> = poise::ApplicationContext<'a, Data, Error>;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionOpen {
    pub id: u32,
    pub date: DateTime<Local>,
    pub ticker: String,
    pub strike: f64,
    pub expiry: DateTime<Local>,
    pub premium: f64,
    pub quantity: u16,
    pub status: String,
    pub close_id: Option<u32>,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionClose {
    pub id: u32,
    pub date: DateTime<Local>,
    pub close_type: String,
    pub open_id: u32,
    pub roll_id: u32,
    pub premium: f64,
    pub quantity: u16,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionAssignment {
    pub id: u32,
    pub date: DateTime<Local>,
    pub open_id: u32,
    pub ticker: String,
    pub strike: f64,
    pub quantity: u16,
}
