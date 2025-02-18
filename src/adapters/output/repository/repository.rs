use sqlx::{PgPool, Pool, Postgres};

#[derive(Clone)]
pub struct Repository {
    pub pg_pool: Pool<Postgres>,
}

impl Repository {
    pub async fn new(pg_pool: PgPool) -> Result<Repository, anyhow::Error> {
        Ok(Repository { pg_pool })
    }
}
