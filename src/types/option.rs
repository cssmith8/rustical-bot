use chrono::prelude::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionOpen {
    pub date: DateTime<Utc>,
    pub open_type: String,
    pub ticker: String,
    pub strike: f64,
    pub expiry: DateTime<Utc>,
    pub premium: f64,
    pub quantity: u16,
    pub status: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OptionClose {
    pub date: DateTime<Utc>,
    pub close_type: String,
    pub premium: f64,
}