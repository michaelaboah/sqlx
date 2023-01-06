use sql::{
    database_setup::sql_setup::initialize_db,
    entities::{struct_parsing::parse_item, structs::Item},
};
use std::fs;
mod sql;

const DB_PATH: &str = "src/resources/sqlite-internal.db";

fn main() {
    // match initialize_db(DB_PATH) {
    //     Ok(_) => (),
    //     Err(err) => println!("Error: {}", err),
    // };

    let yamaha = fs::read_to_string("src/test.json").unwrap();

    let test = parse_item::<Item>(yamaha.as_str());

    println!("{:#?}", test.unwrap())
}
