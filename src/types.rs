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

    pub async fn display(&self) -> String {
        let open = &self.open;
        let close = &self.close;
        let open_date = format!(
            "{}/{}/{}",
            open.date.month(),
            open.date.day(),
            open.date.year() % 100
        );
        let expiry_date = format!(
            "{}/{}/{}",
            open.expiry.month(),
            open.expiry.day(),
            open.expiry.year() % 100
        );
        let close_info = match close {
            Some(c) => format!(
                "Closed on {}/{}/{} at premium ${}",
                c.date.month(),
                c.date.day(),
                c.date.year() % 100,
                c.premium
            ),
            None => "Still open".to_string(),
        };
        format!(
            "{} {} ${} {}\n\
            Premium: ${}\n\
            Quantity: {}\n\
            Opened on: {}\n\
            Status: {}",
            open.ticker,
            expiry_date,
            open.strike,
            open.open_type,
            open.premium,
            open.quantity,
            open_date,
            close_info
        )
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
        (self.contracts.iter().map(|c| c.aggregate_premium()).sum::<f64>() * 100.0).round() / 100.0
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

    pub fn get_status(&self) -> String {
        self.contracts[self.contracts.len() - 1].open.status.clone()
    }

    pub fn display(&self) -> String {
        let rolls = self.num_rolls();
        let option = &self.contracts[self.contracts.len() - 1].open;
        let date: String = option.expiry.month().to_string()
            + "/"
            + &option.expiry.day().to_string()
            + "/"
            + &(option.expiry.year() % 100).to_string();
        let opendate: String = option.date.month().to_string()
            + "/"
            + &option.date.day().to_string()
            + "/"
            + &(option.date.year() % 100).to_string();
        //capitalize the open type first letter
        let open_type = option
            .open_type
            .chars()
            .next()
            .unwrap()
            .to_uppercase()
            .collect::<String>();
        let rolls_string = if rolls > 0 {
            format!("-# *Rolled {} times*\n", rolls)
        } else {
            "".to_string()
        };
        let title_string = format!(
            "{} {} ${} {}",
            option.ticker, date, option.strike, open_type
        );
        let info_string = format!(
            "*Opened on {}*\nPremium: ${}\nQuantity: {}",
            opendate,
            self.aggregate_premium(),
            option.quantity
        );
        return format!("# {title_string}\n{rolls_string}{info_string}\n");
    }
}
