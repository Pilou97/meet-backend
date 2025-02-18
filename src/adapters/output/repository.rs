use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Clone)]
pub struct Repository {
    pg_pool: Pool<Postgres>,
}

impl Repository {
    pub async fn new(uri: &url::Url) -> Result<Repository, sqlx::Error> {
        let pg_pool = PgPoolOptions::new()
            .max_connections(8)
            .connect(uri.as_str())
            .await?;

        Ok(Repository { pg_pool })
    }
}
