pub fn discord_token() -> String {
    std::env::var("DISCORD_TOKEN").unwrap_or_else(|_| "default_token".into())
}

pub fn data_path() -> String {
    std::env::var("DATA_PATH").unwrap_or_else(|_| "data/".into())
}

pub fn static_path() -> String {
    std::env::var("STATIC_PATH").unwrap_or_else(|_| "static/".into())
}

pub fn laptop() -> String {
    std::env::var("LAPTOP").unwrap_or_else(|_| "0".into())
}