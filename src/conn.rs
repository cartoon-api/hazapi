// External USEs
use sqlx::{PgPool, postgres::PgPoolOptions};
use dotenvy::dotenv;

// Internal USEs
use std::env;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    dotenv().ok();
    let db_url: String = env::var("DATABASE_URL")
        .expect("DATABASE_URL Not Defined");

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await?;
    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    type AsyncError = Box<dyn std::error::Error + Send + Sync + 'static>;
    #[tokio::main]
    async fn test_create_pool() -> Result<(), AsyncError> {
        let pool: PgPool = create_pool().await?;
        Ok(assert!(!pool.is_closed()))
    }

    #[tokio::test]
    async fn test_select_status_in_the_row() -> Result<(), AsyncError> {
        let pool: PgPool = create_pool().await?;
        let status: String = sqlx::query_scalar!(
                "SELECT status FROM tests WHERE id = (SELECT id ORDER BY id DESC LIMIT 1)"
            )
            .fetch_one(&pool)
            .await?;

        Ok(assert_eq!("ok", status))
    }

    #[tokio::test]
    async fn test_insert_row() -> Result<(), AsyncError> {
        let pool: PgPool = create_pool().await?;
        let id: i32 = sqlx::query_scalar!(
            "INSERT INTO tests (description, status) VALUES ('Teste de Conexão', 'ok') RETURNING id"
            )
            .fetch_one(&pool)
            .await?;

        Ok(assert!(id > 1))
    }

    #[tokio::test]
    async fn test_delete_row() -> Result<(), AsyncError> {
        let pool: PgPool = create_pool().await?;
        let _ = sqlx::query(
            "DELETE FROM tests WHERE id = (SELECT id ORDER BY id DESC LIMIT 1)"
            )
            .execute(&pool)
            .await?;

        Ok(())
    }

}
