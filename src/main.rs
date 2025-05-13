use sqlx::{postgres::PgPoolOptions, Row};
use dotenv::dotenv;
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

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    println!("DATABASE_URL: {}", database_url);

    let connect_options = sqlx::postgres::PgConnectOptions::new()
        .host("aws-0-ap-southeast-1.pooler.supabase.com")
        .port(6543)
        .username("postgres.turmgklvnzdtzpupiojs")
        .password("Noire2007")
        .database("postgres")
        .statement_cache_capacity(0);

    println!("Connecting with user: {:?}", connect_options.get_username());

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect_with(connect_options)
        .await?;

    let row = sqlx::raw_sql(&sanitize_sql_input("SELECT 1 as value"))
        .fetch_one(&pool)
        .await?;

    let value: i32 = row.try_get("value")?;

    println!("Supabase says: {}", value);
    Ok(())
}
