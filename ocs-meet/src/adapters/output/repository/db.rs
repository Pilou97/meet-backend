use anyhow::Context;
use sqlx::{migrate, PgPool, Pool, Postgres};

#[derive(Clone)]
pub struct Repository {
    pub pg_pool: Pool<Postgres>,
}

impl Repository {
    pub async fn new(pg_pool: PgPool) -> Result<Repository, anyhow::Error> {
        migrate!("./migrations")
            .run(&pg_pool)
            .await
            .context("Cannot run migration")?;

        Ok(Repository { pg_pool })
    }
}
