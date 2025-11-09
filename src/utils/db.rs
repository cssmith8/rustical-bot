use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

pub fn create_or_open_db(path: String) -> PickleDb {
    //pickle db
    //2
    //3
    // third
    let opendb = match PickleDb::load(
        path.clone(),
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    ) {
        Ok(opendb) => opendb,
        Err(_e) => {
            println!("Creating new db at: {}", path);
            PickleDb::new(
                path.clone(),
                PickleDbDumpPolicy::AutoDump,
                SerializationMethod::Json,
            )
        }
    };
    opendb
}
