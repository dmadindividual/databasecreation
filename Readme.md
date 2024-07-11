### README

# SQLite Database Creation with `sqlx` in Rust

## Overview

This Rust program demonstrates how to create and interact with an SQLite database using the `sqlx` library and the `async-std` runtime. The program includes functions to create the database schema, insert data into tables, and perform basic queries. It ensures the database is created only if it does not already exist.

## Features

- **Database Creation**: Checks if the database exists and creates it if necessary.
- **Schema Creation**: Defines and creates tables within the SQLite database.
- **Data Insertion**: Inserts data into the tables.
- **Async Operations**: Uses asynchronous operations for database connections and queries.

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust package manager)
- SQLite installed on your system

## Dependencies

This project uses the following dependencies:

- `sqlx`: A SQL toolkit for Rust that supports compile-time checked queries.
- `async-std`: An asynchronous version of the Rust standard library.

Add these dependencies to your `Cargo.toml` file:

```toml
[dependencies]
sqlx = { version = "0.5", features = ["runtime-async-std", "macros", "sqlite"] }
async-std = "1.10"
```

## Getting Started

1. **Clone the Repository**:
   ```sh
   git clone https://github.com/your-username/sqlite-db-example.git
   cd sqlite-db-example
   ```

2. **Setup and Run**:
   Ensure you have Rust and Cargo installed. Then, run the project using Cargo:
   ```sh
   cargo run
   ```

## Code Explanation

### `create_schema`

The `create_schema` function connects to the SQLite database and creates the necessary tables if they do not already exist.

```rust
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
    pool.close().await?;
    Ok(result)
}
```

### `main`

The `main` function initializes the database creation process.

```rust
#[async_std::main]
async fn main() {
    let db_url = String::from("sqlite://sqlite.db");
    database_creation(&db_url).await;
}
```

### `database_creation`

The `database_creation` function checks if the database exists. If it does not, it creates the database and calls the `create_schema` function.

```rust
async fn database_creation(db_url: &str) {
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
        Sqlite::create_database(&db_url).await.unwrap();

        match create_schema(&db_url).await {
            Ok(_) => println!("Database created successfully").await,
            Err(e) => println!("Error: {}", e).await
        }
    } else {
        println!("Database already exists").await;
    }

    make_query(&db_url).await;
}
```

### `make_query`

The `make_query` function inserts data into the `settings` table.

```rust
async fn make_query(db_url: &str) {
    let pool = SqlitePool::connect(&db_url).await.unwrap();
    let query = "INSERT INTO settings (description) VALUES (?1)";
    let result = sqlx::query(&query).bind("testing").execute(&pool).await.unwrap();

    pool.close().await.unwrap();
    println!("{:?}", result);
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [sqlx](https://github.com/launchbadge/sqlx) for providing a powerful and flexible SQL toolkit for Rust.
- [async-std](https://async.rs/) for the asynchronous standard library for Rust.