use std::env;
use dotenvy::dotenv;
use sqlx::{self, postgres::PgPoolOptions, PgPool, Pool, Postgres};

pub type DbPool = Pool<Postgres>;

pub async fn establish_connection() -> PgPool {
  dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}