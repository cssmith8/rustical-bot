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
    position.contracts[position.contracts.len() - 1]
        .open
        .status
        .clone()
}

//function to display a float as a string with 2 decimal places and a dollar sign
pub fn display_dollars(amount: f64) -> String {
    format!("${:.2}", amount)
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
