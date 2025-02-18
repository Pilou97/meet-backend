use anyhow::Context;
use sqlx::{migrate, postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone)]
pub struct Repository {
    pg_pool: Pool<Postgres>,
}

impl Repository {
    pub async fn new(uri: &url::Url) -> Result<Repository, anyhow::Error> {
        let pg_pool = PgPoolOptions::new()
            .max_connections(8)
            .connect(uri.as_str())
            .await
            .context("Cannot connect to postgresql database")?;

        migrate!("./migrations")
            .run(&pg_pool)
            .await
            .context("The database migration failed")?;

        Ok(Repository { pg_pool })
    }
}
