use chrono::prelude::*;
use crate::types::option::OptionOpen;
use crate::types::option::OptionClose;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Contract {
    pub open: OptionOpen,
    pub close: Option<OptionClose>,
}

impl Contract {

    pub fn clone(&self) -> Contract {
        Contract {
            open: OptionOpen {
                date: self.open.date,
                open_type: self.open.open_type.clone(),
                ticker: self.open.ticker.clone(),
                strike: self.open.strike,
                expiry: self.open.expiry,
                premium: self.open.premium,
                quantity: self.open.quantity,
                status: self.open.status.clone(),
            },
            close: self.close.as_ref().map(|c| OptionClose {
                date: c.date,
                close_type: c.close_type.clone(),
                premium: c.premium,
            }),
        }
    }

    pub fn aggregate_premium(&self) -> f64 {
        match &self.close {
            Some(close) => self.open.premium - close.premium,
            None => self.open.premium,
        }
    }

    pub fn option_type(&self) -> String {
        self.open.open_type.clone()
    }

    pub fn ticker(&self) -> String {
        self.open.ticker.clone()
    }

    pub fn strike(&self) -> f64 {
        self.open.strike
    }

    pub fn expiry(&self) -> DateTime<Utc> {
        self.open.expiry
    }

    pub fn quantity(&self) -> u16 {
        self.open.quantity
    }

    pub fn status(&self) -> String {
        self.open.status.clone()
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
        let unixopendate = open.date.timestamp();
        let expiry_date = format!(
            "{}/{}/{}",
            open.expiry.month(),
            open.expiry.day(),
            open.expiry.year() % 100
        );
        let unixexpirydate = open.expiry.timestamp();
        let close_info = match close {
            Some(c) => format!(
                "Closed <t:{}:R> ({}/{}/{}) at premium ${}",
                c.date.timestamp(),
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
            Opened <t:{}:R> ({})\n\
            Expires <t:{}:R> ({})\n\
            Status: {}",
            open.ticker,
            expiry_date,
            open.strike,
            open.open_type,
            open.premium,
            open.quantity,
            unixopendate,
            open_date,
            unixexpirydate,
            expiry_date,
            close_info
        )
    }
}