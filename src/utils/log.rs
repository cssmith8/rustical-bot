use crate::types::dblog::DBLog;
use crate::types::types::Error;
use crate::utils::db::create_or_open_db;
use chrono::Utc;

#[allow(dead_code)]
pub fn log(message: String) -> Result<(), Error> {
    let mut db = create_or_open_db("data/logs.db".to_string());
    if !db.lexists("logs") {
        db.lcreate("logs")?;
    }
    db.ladd(
        "logs",
        &DBLog {
            timestamp: Utc::now(),
            message,
        },
    )
    .ok_or_else(|| Error::from("Failed to add log to database"))?;
    Ok(())
}

pub fn load_all_logs() -> Result<Vec<DBLog>, Error> {
    let db = create_or_open_db("data/logs.db".to_string());

    let mut all_logs: Vec<DBLog> = Vec::new();
    for item_iter in db.liter("logs") {
        let db_log = item_iter.get_item::<DBLog>().unwrap();
        all_logs.push(db_log);
    }
    Ok(all_logs)
}
