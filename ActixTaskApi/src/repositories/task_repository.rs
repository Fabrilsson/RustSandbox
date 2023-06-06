use crate::database::db::{DBError, create_pool};
use crate::model::task::{Task};

use sqlx::{PgPool};

pub struct TaskRepository {
    pool: PgPool
}

impl TaskRepository {
    pub async fn new() -> TaskRepository {
        TaskRepository { pool: create_pool().await.expect("Failed to create pool") }
    }

    pub async fn insert_task(&self, task: Task) -> Result<(), DBError> {
        Ok(())
    }
}