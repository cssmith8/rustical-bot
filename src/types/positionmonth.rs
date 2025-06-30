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

    pub fn display(&self) -> String {
        format!("{} {}\nPosition: [\n{}]\nGain: {}\nInvestment: {}", &self.month_name(), &self.year, self.position.display(), self.gain, self.investment)
    }
}