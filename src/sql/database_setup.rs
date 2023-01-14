pub mod sql_setup {

    use std::fs;

    // use futures::TyStreamExt;
    use crate::sql::{
        entities::{creation_structs::CreateItem, structs::Item},
        queries::{
            find::{find_similar_item, fuzzy_find_single_item},
            insertion::insert_item,
            schema::{ITEM_RELATIONSHIPS, PRAGMA_QUERIES, TABLE_QUERIES},
        },
    };
    use sqlx::{
        sqlite::{self},
        Connection, SqliteConnection,
    };
    #[tokio::main]
    //Should change path from &str to Path or PathBuf
    pub async fn initialize_db(path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut conn = get_connection(path).await.expect("Problem with pool");
        //Create tables
        // for table in TABLE_QUERIES.iter() {
        //     sqlx::query(table).execute(&mut conn).await?;
        // }

        // //Create Relationships
        // for relation in ITEM_RELATIONSHIPS.iter() {
        //     sqlx::query(relation).execute(&mut conn).await?;
        // }
        sqlx::query_file!("src/resources/internal-schema.sql");
        // //Pragma Checks
        // for routine in PRAGMA_QUERIES.iter() {
        //     sqlx::query(routine).execute(&mut conn).await?;
        // }

        // let json_string = fs::read_to_string("src/test.json").expect("Err with file read");
        // let console_item: Item = serde_json::from_str(&json_string).expect("Err with parse");
        // insert_item(&console_item, path).await?;

        // let raw_string =
        //     fs::read_to_string("src/test-multiple-items.json").expect("error with string reading");

        // let items: Vec<Item> = serde_json::from_str(&raw_string).expect("Error with json parse");

        // for item in items.iter() {
        //     insert_item(&item, path).await?;
        // }
        let thing = fuzzy_find_single_item("QL5", path).await;

        // let test = find_similar_item("D", path).await;
        // let fetch_users = sqlx::query_as!(CreateItem, "SELECT * FROM item")
        //     .fetch_all(&mut conn)
        //     .await?;

        // println!("{:#?}", fetch_users);
        Ok(())
    }

    pub async fn get_connection(path: &str) -> Result<SqliteConnection, sqlx::Error> {
        let sql_connection = sqlite::SqliteConnection::connect(path).await;
        match sql_connection {
            Ok(conn) => Ok(conn),
            Err(conn_err) => Err(conn_err),
        }
    }
}
