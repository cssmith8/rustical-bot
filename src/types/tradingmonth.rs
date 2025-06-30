//use chrono::prelude::*;
use crate::types::positionmonth::PositionMonth;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TradingMonth {
    pub year: i32,
    pub month: u32,
    pub gain: f64,
    pub investment: f64
}

impl TradingMonth {

    pub fn month_name(&self) -> String {
        return match self.month {
            1 => "January".to_string(),
            2 => "February".to_string(),
            3 => "March".to_string(),
            4 => "April".to_string(),
            5 => "May".to_string(),
            6 => "June".to_string(),
            7 => "July".to_string(),
            8 => "August".to_string(),
            9 => "September".to_string(),
            10 => "October".to_string(),
            11 => "November".to_string(),
            12 => "December".to_string(),
            _ => panic!("Invalid month"),
        };
    }

    pub fn combine(&mut self, positionmonth: PositionMonth) {
        self.gain += positionmonth.gain;
        self.investment += positionmonth.investment;
    }

    pub fn daily_return_rate(&self) -> f64 {
        self.gain / self.investment * 100.0
    }

    #[allow(dead_code)]
    pub fn display(&self) -> String {
        format!("{} {}\nGain: {}\nInvestment: {}", &self.month_name(), &self.year, self.gain, self.investment)
    }
}