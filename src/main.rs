use sql::{database_setup::sql_setup::initialize_db, entities::stucts::Item};
use std::fs;
mod sql;

const DB_PATH: &str = "src/resources/sqlite-internal.db";

fn main() {
    // match initialize_db(DB_PATH) {
    //     Ok(_) => (),
    //     Err(err) => println!("Error: {}", err),
    // };

    let yamaha = fs::read_to_string("src/test.json").unwrap();
    let converted: Option<Item> = match serde_json::from_str(yamaha.as_str()) {
        Ok(con) => Some(con),
        Err(err) => {
            println!("{err}");
            None
        }
    };
    let new = converted.unwrap();
    println!("{:#?}", new.console.unwrap())
}
