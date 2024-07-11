use std::result::Result;
use async_std::task;
use sqlx::{sqlite::SqliteQueryResult, Sqlite, SqlitePool, migrate::MigrateDatabase};

async fn create_schema(db_url: &str) -> Result<SqliteQueryResult, sqlx::Error> {
    let pool = SqlitePool::connect(&db_url).await?;
    let query = "
        PRAGMA foreign_keys = ON;
        CREATE TABLE IF NOT EXISTS settings (
            settings_id INTEGER PRIMARY KEY NOT NULL,
            description TEXT NOT NULL,
            created_on DATETIME DEFAULT (datetime('now', 'localtime')),
            updated_on DATETIME DEFAULT (datetime('now', 'localtime')),
            done BOOLEAN NOT NULL DEFAULT 0
        );
        CREATE TABLE IF NOT EXISTS project (
            project_id INTEGER PRIMARY KEY NOT NULL
        );
    ";
    let result = sqlx::query(&query).execute(&pool).await?;
    pool.close().await;
    Ok(result)
}

#[async_std::main]
async fn main() {
    let db_url = String::from("sqlite://sqlite.db");
    database_creation(&db_url).await;
}

async fn database_creation(db_url: &str) {
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await.unwrap();

        match create_schema(&db_url).await {
            Ok(_) => println!("Database created successfully"),
            Err(e) => println!("Error: {}", e)
        }
    } else {
        println!("Database already exists");
    }

    make_query(&db_url).await;
}

async fn make_query(db_url: &str) {
    let pool = SqlitePool::connect(&db_url).await.unwrap();
    let query = "INSERT INTO settings (description) VALUES (?1)";
    let result = sqlx::query(&query).bind("testing").execute(&pool).await.unwrap();

    pool.close().await;
    println!("{:?}", result);
}
