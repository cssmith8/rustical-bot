pub fn discord_token() -> String {
    let bot_name = std::env::var("BOT").expect("BOT environment variable not set");
    match bot_name.to_lowercase().as_str() {
        "rustical" => std::env::var("RUSTICAL").unwrap_or_else(|_| "default_token".into()),
        "moneymouth" => std::env::var("MONEYMOUTH").unwrap_or_else(|_| "default_token".into()),
        "fretter" => std::env::var("FRETTER").unwrap_or_else(|_| "default_token".into()),
        _ => {
            panic!("Unknown bot specified in .env")
        }
    }
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

pub fn clear_password() -> String {
    std::env::var("CLEAR_PASSWORD").unwrap_or_else(|_| "default_clear_password".into())
}
