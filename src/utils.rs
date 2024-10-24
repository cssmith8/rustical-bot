use crate::types::Position;
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

//function to check the status of the last option open in a position
pub fn get_position_status(position: Position) -> String {
    let last_index = position.contracts.len() - 1;
    let open_status = position.contracts[last_index].open.status.clone();
    open_status
}

// pub fn int_list_replace(db: &mut PickleDb, name: &str, index: usize, position: i32) {
//     //empty vector
//     let mut vec: Vec<i32> = Vec::new();
//     // iterate over the items in list1
//     for item_iter in db.liter(name) {
//         vec.push(item_iter.get_item::<i32>().unwrap());
//     }
//     //replace element at index
//     vec[index] = position;

//     db.lrem_list(name).unwrap();
//     // create a new list
//     db.lcreate(name).unwrap();
//     // add a bunch of numbers to the list
//     db.lextend(name, &vec).unwrap();
// }
