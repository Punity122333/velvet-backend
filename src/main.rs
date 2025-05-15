use dotenv::dotenv;
use sqlx::{postgres::PgConnectOptions, Pool, Postgres, Row};
use std::env;

fn sanitize_sql_input(input: &str) -> String {
    let dangerous_keywords = [
        "DROP", "DELETE", "UPDATE", "INSERT", "ALTER", "TRUNCATE", 
        "CREATE", "EXEC", "FETCH", "MERGE", "REPLACE", "GRANT", "REVOKE", "DECLARE",
        "UNION", "BEGIN", "COMMIT", "ROLLBACK", "ROLLBACK", "WITH", "EXPLAIN",
    ];

    let input_upper = input.to_uppercase();

    if dangerous_keywords.iter().any(|&keyword| input_upper.contains(keyword)) {
        panic!("Invalid input: {} (contains dangerous SQL keyword)", input);
    }

    if input.contains("--") || input.contains("/*") || input.contains("*/") {
        panic!("Invalid input: {} (contains SQL comment)", input);
    }

    if input.chars().any(|c| !c.is_alphanumeric() && c != '_' && c != ' ' && c != '-') {
        panic!("Invalid input: {} (contains unsafe characters)", input);
    }

    let sanitized_input = input.replace("'", "''");

    sanitized_input
}

async fn supabase_test(pool: &Pool<Postgres>) -> i32 {
    let rows = sqlx::query("SELECT 1 AS value")
        .bind(sanitize_sql_input("SELECT 1 AS value")) 
        .fetch_all(pool)
        .await
        .expect("Failed to execute query");
    let row = rows.into_iter().next().expect("No rows returned");
    let value: i32 = row.try_get("value").expect("Failed to get value");

    value
}
async fn init_env() -> Pool<Postgres> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    println!("DATABASE_URL: {}", database_url);

    let options = database_url
        .parse::<PgConnectOptions>()
        .expect("Invalid database URL")
        .statement_cache_capacity(0);

    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .expect("Failed to create connection pool")
}

#[tokio::main]
async fn main() {
    let pool = init_env().await;

    let value = supabase_test(&pool).await;
    println!("Supabase says: {}", value);

    let value = supabase_test(&pool).await;
    println!("Supabase says: {}", value);
}
