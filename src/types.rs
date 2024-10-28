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

impl Contract {
    pub fn aggregate_premium(&self) -> f64 {
        match &self.close {
            Some(close) => self.open.premium - close.premium,
            None => self.open.premium,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Position {
    pub contracts: Vec<Contract>,
}

impl Position {
    pub fn aggregate_premium(&self) -> f64 {
        self.contracts.iter().map(|c| c.aggregate_premium()).sum()
    }

    pub fn num_rolls(&self) -> usize {
        self.contracts.len() - 1
    }
}
