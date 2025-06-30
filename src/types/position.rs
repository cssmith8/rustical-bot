use chrono::{prelude::*, Datelike, Duration, NaiveDate};
use crate::types::contract::Contract;
use crate::types::positionmonth::PositionMonth;

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

    pub fn generate_distributed_months(&self) -> Vec<PositionMonth> {
        let startdate = &self.get_first_contract().open.date;
        let enddate = if self.get_final_contract().close.is_some() {
            &self.get_final_contract().close.as_ref().unwrap().date
        } else {
            &self.get_final_contract().expiry()
        };

        let mut current = startdate.naive_utc().date();
        let enddate_naive_dt = enddate.naive_utc();
        let enddate_naive = enddate_naive_dt.date();
        let total_days = (enddate_naive - current).num_days() + 1;

        let mut profit_months = Vec::new();

        while current <= enddate_naive {
            let year = current.year();
            let month = current.month();
            let month_start = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
            let next_month = if month == 12 {
                NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
            } else {
                NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
            };
            let month_end = (next_month - Duration::days(1)).min(enddate_naive);

            let range_start = current.max(month_start);
            let range_end = month_end.min(enddate_naive);
            let days_in_month = (range_end - range_start).num_days() + 1;

            let fraction = days_in_month as f64 / total_days as f64;

            profit_months.push(PositionMonth {
                year: year,
                month: month,
                position: self.clone(),
                gain: &self.gain() * fraction,
                investment: self.investment() * days_in_month as f64
            });
            current = next_month;
        }
        profit_months
    }

    pub fn generate_taxable_months(&self) -> Vec<PositionMonth> {
        let startdate = &self.get_first_contract().open.date;
        let enddate = if self.get_final_contract().close.is_some() {
            &self.get_final_contract().close.as_ref().unwrap().date
        } else {
            &self.get_final_contract().expiry()
        };

        let mut current = startdate.naive_utc().date();
        let enddate_naive_dt = enddate.naive_utc();
        let enddate_naive = enddate_naive_dt.date();
        let total_days = (enddate_naive - current).num_days() + 1;

        let mut profit_months = Vec::new();

        while current <= enddate_naive {
            let year = current.year();
            let month = current.month();
            let month_start = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
            let next_month = if month == 12 {
                NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap()
            } else {
                NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap()
            };
            let month_end = (next_month - Duration::days(1)).min(enddate_naive);

            let range_start = current.max(month_start);
            let range_end = month_end.min(enddate_naive);
            let days_in_month = (range_end - range_start).num_days() + 1;

            let fraction = days_in_month as f64 / total_days as f64;
            
            let gain = if year == enddate_naive.year() && month == enddate_naive.month() {
                self.gain()
            } else {
                0.0
            };

            profit_months.push(PositionMonth {
                year: year,
                month: month,
                position: self.clone(),
                gain,
                investment: self.investment() * days_in_month as f64
            });
            current = next_month;
        }
        profit_months
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