use sqlx::{Executor, SqlitePool};
use std::time::Duration;

pub fn default_string() -> String {
    "something".to_string()
}

pub fn particular_value() -> String {
    "interesting".to_string()
}

pub fn interesting_appendage(input: &str) -> String {
    format!("{} {}", input, particular_value())
}

pub async fn some_async_stringer() -> String {
    tokio::time::sleep(Duration::from_millis(150)).await;
    "quiet screaming".to_string()
}

pub async fn init_db() -> SqlitePool {
    SqlitePool::connect("sqlite::memory:").await.unwrap()
}

pub async fn create_tables(db: &SqlitePool) {
    db.execute("CREATE TABLE test(id SERIAL PRIMARY KEY, value TEXT NOT NULL);")
        .await
        .unwrap();
}

pub async fn insert_data(db: &SqlitePool) {
    db.execute("INSERT INTO test(id, value) VALUES (1, 'quiet screaming');")
        .await
        .unwrap();
}

pub async fn drop_tables(db: &SqlitePool) {
    db.execute("DROP TABLE test;").await.unwrap();
}

#[derive(sqlx::FromRow)]
pub struct Test {
    pub value: String,
    pub id: i64,
}

pub async fn find_by_id(db: &SqlitePool, id: i64) -> Option<Test> {
    sqlx::query_as::<_, Test>("SELECT * FROM test WHERE id = ?")
        .bind(id)
        .fetch_one(db)
        .await
        .ok()
}
