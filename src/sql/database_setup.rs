pub mod sql_setup {

    use std::fs;

    // use futures::TyStreamExt;
    use crate::sql::queries::find::fuzzy_find_single_item;
    use sqlx::sqlite::SqlitePoolOptions;
    #[tokio::main]
    //Should change path from &str to Path or PathBuf
    pub async fn initialize_db(path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect(path)
            .await?;
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
        let thing = fuzzy_find_single_item("D20", &pool).await;
        let amp_item = thing.new_from_table(&pool).await;
        let jsonb = serde_json::to_string_pretty(&amp_item).unwrap();
        fs::write("./test_amp.json", jsonb);
        // println!("{:#?}", jsonb);

        // let test = find_similar_item("D", path).await;
        // let fetch_users = sqlx::query_as!(CreateItem, "SELECT * FROM item")
        //     .fetch_all(&mut conn)
        //     .await?;

        // println!("{:#?}", fetch_users);
        Ok(())
    }
}
