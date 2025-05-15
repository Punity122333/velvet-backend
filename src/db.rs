use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use std::env;

pub async fn connect_db() -> sqlx::PgPool {
    dotenv().ok(); // Load environment variables from .env file
    let db_url = env::var("SUPABASE_DB_URL").expect("DB URL not set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("DB connection failed")
}
