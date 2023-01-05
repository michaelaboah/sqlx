pub mod sql_setup {
    use sqlx::{sqlite, Connection, SqliteConnection};
    use tokio;

    use crate::sql::queries::schema::{ITEM_RELATIONSHIPS, PRAGMA_QUERIES, TABLE_QUERIES};

    #[tokio::main]
    //Should change path from &str to Path or PathBuf
    pub async fn initialize_db(path: &str) -> Result<(), Box<dyn std::error::Error>> {
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

        // let row: (i64,) = sqlx::query_as("SELECT $1")
        //     .bind(150_i64)
        //     .fetch_one(&pool)
        //     .await?;
        Ok(())
    }

    async fn get_connection(path: &str) -> Result<SqliteConnection, sqlx::Error> {
        let sql_connection = sqlite::SqliteConnection::connect(path).await;
        match sql_connection {
            Ok(conn) => Ok(conn),
            Err(conn_err) => Err(conn_err),
        }
    }
}
