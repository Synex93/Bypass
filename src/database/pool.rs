use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::OnceCell;

static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn init_pool() -> anyhow::Result<()> {
    let database_url = std::env::var("database_url")?;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let (version,): (String,) = sqlx::query_as("SELECT VERSION()").fetch_one(&pool).await?;
    tracing::info!("database version: {}", version);

    DB_POOL
        .set(pool)
        .map_err(|_| anyhow::anyhow!("failed to set database pool"))?;

    Ok(())
}

pub fn get_pool() -> anyhow::Result<&'static PgPool> {
    DB_POOL
        .get()
        .ok_or_else(|| anyhow::anyhow!("database pool not initialized"))
}
