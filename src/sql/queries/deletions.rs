use sqlx::sqlite::{Sqlite, SqliteQueryResult};
use sqlx::Pool;

pub async fn delete_all_items(pool: &Pool<Sqlite>) -> sqlx::Result<SqliteQueryResult> {
    sqlx::query!("DELETE FROM item;").execute(pool).await
}

pub async fn delete_single_item(id: i64, pool: &Pool<Sqlite>) -> sqlx::Result<SqliteQueryResult> {
    sqlx::query!("DELETE FROM item WHERE id = ?", id)
        .execute(pool)
        .await
}

pub async fn fuzzy_delete_single_item(
    model: &str,
    pool: &Pool<Sqlite>,
) -> sqlx::Result<SqliteQueryResult> {
    let formatted = format!("%{model}%");
    sqlx::query!("DELETE FROM item WHERE model LIKE ?", formatted)
        .execute(pool)
        .await
}
