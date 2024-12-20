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
    pub fn is_closed(&self) -> bool {
        for contract in &self.contracts {
            if contract.close.is_none() {
                return false;
            }
        }
        true
    }
    pub fn aggregate_premium(&self) -> f64 {
        self.contracts.iter().map(|c| c.aggregate_premium()).sum()
    }

    pub fn gain(&self) -> f64 {
        self.aggregate_premium() * self.contracts[0].open.quantity as f64 * 100.0
    }

    pub fn num_rolls(&self) -> usize {
        self.contracts.len() - 1
    }

    pub fn time(&self) -> i64 {
        //for each contract
        let mut time = chrono::Duration::zero();
        for contract in &self.contracts {
            //if the contract is closed
            if let Some(close) = &contract.close {
                //return the time between the open and close
                time += close.date.signed_duration_since(contract.open.date);
            } else {
                //if the contract is open, return the time between the open and now
                time += Local::now().signed_duration_since(contract.open.date);
            }
        }
        if time.num_days() < 1 {
            return 1;
        }
        time.num_days()
    }

    pub fn investment(&self) -> f64 {
        if self.contracts.len() == 0 {
            print!("Error: Empty position");
            return 0.0;
        }
        self.contracts[0].open.strike * self.contracts[0].open.quantity as f64 * 100.0
    }

    pub fn return_on_investment(&self) -> f64 {
        self.gain() / self.investment()
    }
}
