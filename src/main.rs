use sql::database_setup::sql_setup::initialize_db;
mod sql;
use dotenvy::dotenv;
use std::env;

use crate::sql::entities::structs::Item;
// const DB_PATH: &str = "src/resources/sqlite-internal.db";

fn main() {
    dotenv().expect(".env file not found");

    let db_path = env::var("DATABASE_URL").unwrap();
    match initialize_db(&db_path) {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    };

    // println!("{}", &db_path);
}
