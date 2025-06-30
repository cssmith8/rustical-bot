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

    pub fn id(&self) -> String {
        format!("{}-{:02}", self.year, self.month)
    }

    pub fn combine(&mut self, positionmonth: PositionMonth) {
        self.gain += positionmonth.gain;
        self.investment += positionmonth.investment;
    }

    pub fn daily_return_rate(&self) -> f64 {
        self.gain / self.investment * 100.0
    }

    pub fn display_daily_return_rate(&self) -> String {
        let year = self.year;
        let month = self.month;
        let month_name = chrono::NaiveDate::from_ymd_opt(year, month, 1)
            .map(|d| d.format("%B").to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        return format!("{} {}: `{:.2}%`", month_name, year, self.daily_return_rate());
    }

    pub fn display_distributed_gain(&self) -> String {
        let year = self.year;
        let month = self.month;
        let month_name = chrono::NaiveDate::from_ymd_opt(year, month, 1)
            .map(|d| d.format("%B").to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        return format!("{} {}: `${:.2}`", month_name, year, self.gain);
    }
}