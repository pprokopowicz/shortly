use dotenv::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

const DB_ENV_KEY: &'static str = "DATABASE_URL";

pub struct DbConnection {
    pub(super) pool: PgPool,
}

impl DbConnection {
    pub async fn new() -> Result<Self, sqlx::Error> {
        dotenv().ok();

        let db_url = env::var(DB_ENV_KEY).expect("DATABASE_URL must be set");
        let pool = PgPoolOptions::new()
            .max_connections(8)
            .connect(&db_url)
            .await?;

        Ok(DbConnection { pool })
    }
}
