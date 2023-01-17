use crate::sql::entities::creation_structs::CreateItem;
use sqlx::sqlite::Sqlite;
use sqlx::Pool;

pub async fn find_similar_item(model: &str, pool: &Pool<Sqlite>) -> Vec<CreateItem> {
    let formatted = format!("%{model}%");
    let similar_items = sqlx::query_as!(
        CreateItem,
        "SELECT * from item WHERE model LIKE ?",
        formatted
    )
    .fetch_all(pool)
    .await
    .expect("Fetch error at find_one_item.");

    similar_items
}

pub async fn find_all_items(pool: &Pool<Sqlite>) -> Vec<CreateItem> {
    let all_items = sqlx::query_as!(CreateItem, "SELECT * FROM item;")
        .fetch_all(pool)
        .await
        .expect("Fetch error at find_all_items;");
    all_items
}

pub async fn find_single_item(id: i64, pool: &Pool<Sqlite>) -> CreateItem {
    let single_item = sqlx::query_as!(CreateItem, "SELECT * FROM item WHERE id = ?", id)
        .fetch_one(pool)
        .await
        .expect("Fetch error at find_one_item;");
    single_item
}

pub async fn fuzzy_find_single_item(model: &str, pool: &Pool<Sqlite>) -> CreateItem {
    let formatted = format!("%{model}%");
    let single_item = sqlx::query_as!(
        CreateItem,
        "SELECT * FROM item WHERE model LIKE ?",
        formatted
    )
    .fetch_one(pool)
    .await
    .expect("Fetch error at fuzzy_find_one_item;");
    single_item
}
