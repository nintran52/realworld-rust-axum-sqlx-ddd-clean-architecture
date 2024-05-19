use crate::constants::env_key;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub type DbPool = sqlx::Pool<sqlx::Postgres>;

async fn init_pool(database_url: &str) -> DbPool {
    match PgPoolOptions::new()
        .max_connections(10) // Set the max connections if needed
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    }
}

pub async fn establish_connection() -> DbPool {
    dotenv().ok();
    let database_url = env::var(env_key::DATABASE_URL).expect("DATABASE_URL must be set");
    init_pool(&database_url).await
}
