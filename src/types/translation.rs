#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Translation {
    pub abbreviation: String,
    pub definition: String,
}
