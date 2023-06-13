use sqlx::{PgPool};

#[derive(Debug)]
pub struct DBError;

pub async fn create_pool() -> Result<PgPool, sqlx::Error> {
    let pool = PgPool::connect("postgres://postgres:mysecretpassword@localhost/task_manager")
        .await?;
    
    Ok(pool)
}