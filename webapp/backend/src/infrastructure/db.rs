use sqlx::mysql::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;
use std::env;

pub async fn create_pool() -> MySqlPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MySqlPoolOptions::new()
        .max_connections(10000)
        .connect(&database_url)
        .await
        .expect("Failed to create pool")
}
