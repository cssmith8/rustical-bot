use crate::types::position::Position;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PositionMonth {
    pub year: i32,
    pub month: u32,
    pub position: Position,
    pub gain: f64,
    pub investment: f64,
}

impl PositionMonth {
    pub fn clone(&self) -> PositionMonth {
        PositionMonth { 
            year: self.year, 
            month: self.month, 
            position: self.position.clone(), 
            gain: self.gain, 
            investment: self.investment
        }
    }

    pub fn id(&self) -> String {
        format!("{}-{:02}", self.year, self.month)
    }

    /*
    pub fn display(&self) -> String {
        let month_name = chrono::NaiveDate::from_ymd_opt(self.year, self.month, 1)
            .map(|d| d.format("%B").to_string())
            .unwrap_or_else(|| "Unknown".to_string());
        format!("{} {}\nPosition: [\n{}]\nGain: {}\nInvestment: {}", month_name, &self.year, self.position.display(), self.gain, self.investment)
    }
    */
}