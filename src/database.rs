use anyhow::Result;
use sqlx::{Sqlite, Pool};
use tracing::{debug, info, warn, error};

pub async fn create_pool(db_url: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    let pool = Pool::<Sqlite>::connect(&db_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
