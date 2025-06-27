use crate::types::position::Position;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};

pub fn open_option_db(path: String) -> Option<PickleDb> {
    let mut new_flag = false;
    let mut opendb = match PickleDb::load(
        path.clone(),
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
    ) {
        Ok(opendb) => opendb,
        Err(_e) => {
            println!("Creating new db at: {}", path);
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

pub fn position_list_replace(db: &mut PickleDb, name: &str, index: usize, position: Position) {
    //empty vector
    let mut vec: Vec<Position> = Vec::new();
    // iterate over the items in list1
    for item_iter in db.liter(name) {
        vec.push(item_iter.get_item::<Position>().unwrap());
    }
    //replace element at index
    vec[index] = position;

    db.lrem_list(name).unwrap();
    // create a new list
    db.lcreate(name).unwrap();
    db.lextend(name, &vec).unwrap();
}

pub async fn label_display(index: u32, length: u32, string: &String) -> String {
    return format!("-# {}/{}\n{}", index + 1, length, string);
}
