pub mod sql_setup {

    use std::{fs, io::ErrorKind};

    // use futures::TyStreamExt;
    use crate::sql::entities::structs::Item;
    use sqlx::{error::DatabaseError, sqlite::SqlitePoolOptions};
    #[tokio::main]
    //Should change path from &str to Path or PathBuf
    pub async fn initialize_db(path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(path)
            .await?;

        //Ideally
        let schema = fs::read_to_string("src/resources/internal-schema.sql").unwrap();

        sqlx::query(&schema).execute(&pool).await;

        // let raw_string =
        //     fs::read_to_string("src/test-multiple-items.json").expect("error with string reading");

        // let items: Vec<Item> = serde_json::from_str(&raw_string).expect("Error with json parse");

        // insert_multiple_items(items, &pool).await?;

        // let thing = fuzzy_find_single_item("D20", &pool).await;
        // let amp_item = thing.convert_from_row(&pool).await;
        // let jsonb = serde_json::to_string_pretty(&amp_item).unwrap();
        // fs::write("./test_amp.json", jsonb);
        // println!("{:#?}", jsonb);

        // let test = find_similar_item("D", path).await;
        // let fetch_users = sqlx::query_as!(CreateItem, "SELECT * FROM item")
        //     .fetch_all(&mut conn)
        //     .await?;

        // println!("{:#?}", fetch_users);
        Ok(())
    }
}
