use chrono::{DateTime, Utc};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DBLog {
    pub timestamp: DateTime<Utc>,
    pub message: String,
}

impl DBLog {
    pub fn display(&self) -> String {
        format!("[{}]: {}", self.timestamp, self.message)
    }
}
