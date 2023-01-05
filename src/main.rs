use sql::database_setup::sql_setup::initialize_db;
mod sql;

fn main() {
    match initialize_db("src/resources/sqlite-internal.db") {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    };
}
