use chrono::prelude::*;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

pub fn open_option_db(path: String) -> Option<PickleDb> {
    let mut new_flag = false;
    let mut opendb = match PickleDb::load(
        path.clone(),
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    ) {
        Ok(opendb) => opendb,
        Err(e) => {
            println!("Could not load db: {}, creating new one", e.to_string());
            new_flag = true;
            PickleDb::new(
                path.clone(),
                PickleDbDumpPolicy::AutoDump,
                SerializationMethod::Json,
            )
        }
    };
    if new_flag {
        opendb.set("commission", &0.65).unwrap();
        opendb.set("edit_id", &-1).unwrap();
        opendb.lcreate("positions").unwrap();
    }
    Some(opendb)
}