use std::error::Error;

use sqlx::PgPool;
use sqlx::migrate;

pub async fn migrate(conn: &PgPool) -> Result<(), Box<dyn Error>> {
    migrate!()
        .run(conn)
        .await?;

    Ok(())
}