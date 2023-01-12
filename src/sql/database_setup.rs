pub mod sql_setup {

    // use futures::TyStreamExt;
    use crate::sql::{
        entities::{creation_structs::CreateItem, enums::Categories, structs::Item},
        queries::{
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
        // dotenv().expect("init err");
        let mut conn = get_connection(path).await.expect("Problem with pool");
        //Create tables
        for table in TABLE_QUERIES.iter() {
            sqlx::query(table).execute(&mut conn).await?;
        }

        //Create Relationships
        for relation in ITEM_RELATIONSHIPS.iter() {
            sqlx::query(&relation).execute(&mut conn).await?;
        }

        //Pragma Checks
        for routine in PRAGMA_QUERIES.iter() {
            sqlx::query(&routine).execute(&mut conn).await?;
        }
        // let json_string = fs::read_to_string("src/test.json").expect("Err with file read");
        // let console_item: Item = serde_json::from_str(&json_string).expect("Err with parse");
        // insert_item(&console_item, path).await?;
        let fetch_users = sqlx::query_as!(CreateItem, "SELECT * FROM item")
            .fetch_all(&mut conn)
            .await?;
        // .unwrap("Error with TestItem fetch");
        // let res = fetch_users.
        println!("{:#?}", fetch_users);
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
