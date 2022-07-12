use sqlx::{postgres::PgPoolOptions, types::Uuid};
use std::env;

#[derive(sqlx::FromRow)]
struct Challenge {
    id: Uuid,
    challengeId: usize,
    challengeTitle: String,
    challengeDescription: String,
}

#[derive(sqlx::FromRow)]
struct challengeEntry {
    id: Uuid,
    challenge: Uuid,
    entryPersonName: String,
    entryName: String,
}

#[derive(sqlx::FromRow)]
struct challengeSubmission {
    id: Uuid,
    entry: Uuid,
    fileName: String,
    fileFileEnding: String,
    fileCategory: fileCategory,
    fileCreatedOn: String,
}

enum fileCategory {
    image,
    video,
    docx,
    uncategorized,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let key = "PG_PASS";
    let pg_pass = match env::var(key) {
        Ok(v) => v,
        Err(_) => "DEFAULT".to_string(),
    };

    let url = format!("postgres://postgres:{}@172.16.3.22:5432/wonders", pg_pass);

    let _pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;

    return Ok(());
}
