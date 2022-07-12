use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let key = "PG_PASS";
    let pg_pass = match env::var(key) {
        Ok(v) => v,
        Err(e) => "DEFAULT".to_string(),
    };

    let url = format!("postgres://postgres:{}@172.16.3.22:5432/wonders", pg_pass);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    println!("Hello, world!");

    return Ok(());
}
