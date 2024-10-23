use sqlx::MySqlPool;

pub async fn database_connection() -> Result<MySqlPool, sqlx::Error> {
    MySqlPool::connect(&std::env::var("MYSQL_URI").unwrap()).await
}