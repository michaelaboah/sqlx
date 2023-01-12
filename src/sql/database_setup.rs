pub mod sql_setup {

    use std::fs;

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
        Connection, FromRow, SqliteConnection,
    };

    #[derive(Debug, Default, sqlx::FromRow, PartialEq)]
    pub struct TestItem {
        pub id: i64,
        pub created_at: String,
        pub updated_at: String,
        pub public_notes: Option<String>,
        pub cost: f64,
        pub weight: f64,
        pub dimensions: Option<String>,
        pub model: String,
        pub category: i64,
        pub amplifier_item_id: Option<i64>,
        pub console_item_id: Option<i64>,
        pub computer_item_id: Option<i64>,
        pub processor_item_id: Option<i64>,
        pub network_item_id: Option<i64>,
        pub microphone_item_id: Option<i64>,
        pub radio_item_id: Option<i64>,
        pub speaker_item_id: Option<i64>,
        pub monitoring_item_id: Option<i64>,
        pub notes: Option<String>,
        pub searchable_model: Option<String>,
    }

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
