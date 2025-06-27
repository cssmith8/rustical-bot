use chrono::prelude::*;
use crate::types::contract::Contract;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Position {
    pub contracts: Vec<Contract>,
}

impl Position {

    pub fn clone(&self) -> Position {
        Position {
            contracts: self.contracts.iter().map(|c| c.clone()).collect(),
        }
    }

    pub fn option_type(&self) -> String {
        self.get_first_contract().option_type().clone()
    }

    pub fn is_closed(&self) -> bool {
        let final_contract = self.get_final_contract();
        final_contract.close.is_some()
            || matches!(final_contract.status().as_str(), "assigned" | "expired" | "rolled")
    }

    pub fn get_final_contract(&self) -> &Contract {
        if self.contracts.is_empty() {
            panic!("Error: Empty position");
        }
        &self.contracts[self.contracts.len() - 1]
    }

    pub fn get_first_contract(&self) -> &Contract {
        if self.contracts.is_empty() {
            panic!("Error: Empty position");
        }
        &self.contracts[0]
    }

    pub fn aggregate_premium(&self) -> f64 {
        (self.contracts.iter().map(|c| c.aggregate_premium()).sum::<f64>() * 100.0).round() / 100.0
    }

    pub fn get_ticker(&self) -> String {
        self.get_final_contract().ticker()
    }

    pub fn gain(&self) -> f64 {
        self.aggregate_premium() * self.contracts[0].open.quantity as f64 * 100.0
    }

    pub fn num_rolls(&self) -> usize {
        self.contracts.len() - 1
    }

    pub fn get_status(&self) -> String {
        self.get_final_contract().status().clone()
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
                time += Utc::now().signed_duration_since(contract.open.date);
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
        self.get_final_contract().strike() * self.get_final_contract().quantity() as f64 * 100.0
    }

    pub fn return_on_investment(&self) -> f64 {
        self.gain() / self.investment()
    }

    pub fn investment_time(&self) -> f64 {
        return self.investment() * self.time() as f64;
    }

    pub fn display(&self) -> String {
        let rolls = self.num_rolls();
        let option = &self.get_final_contract().open;
        let date: String = option.expiry.month().to_string()
            + "/"
            + &option.expiry.day().to_string()
            + "/"
            + &(option.expiry.year() % 100).to_string();
        let open_date: String = option.date.month().to_string()
            + "/"
            + &option.date.day().to_string()
            + "/"
            + &(option.date.year() % 100).to_string();
        let unix_expiry_time = option.expiry.timestamp();
        let unix_open_time = option.date.timestamp();
        let expire_format = if option.expiry > Utc::now() {
            "Expires".to_string()
        } else {
            "Expired".to_string()
        };
        //capitalize the open type first letter
        let open_type = option
            .open_type
            .chars()
            .next()
            .unwrap()
            .to_uppercase()
            .collect::<String>();
        let rolls_string = if rolls > 0 {
            let times_str = if rolls == 1 { "time" } else { "times" };
            format!("-# *Rolled {} {}*\n", rolls, times_str)
        } else {
            "".to_string()
        };
        let title_string = format!(
            "{} {} ${} {}",
            option.ticker, date, option.strike, open_type
        );
        let info_string = format!(
            "Opened <t:{}:R> ({})\n{} <t:{}:R>\nPremium: ${}\nQuantity: {}",
            unix_open_time,
            open_date,
            expire_format,
            unix_expiry_time,
            self.aggregate_premium(),
            option.quantity
        );
        return format!("# {title_string}\n{rolls_string}{info_string}\n");
    }
}