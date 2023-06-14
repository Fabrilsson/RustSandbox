use crate::database::db::{DBError, create_pool};
use crate::model::task::{Task};

use sqlx::{PgPool};
use chrono::DateTime;

#[derive(Clone)]
pub struct TaskRepository {
    pub pool: PgPool
}

impl TaskRepository {
    pub async fn new() -> TaskRepository {
        TaskRepository { pool: create_pool().await.expect("Failed to create pool") }
    }

    pub async fn migrate(&self) -> Result<(), DBError> {
        sqlx::migrate!("./migrations").run(&self.pool).await.expect("Failed to execute migrations");
        Ok(())
    }

    pub async fn insert_task(&self, task: Task) -> Result<(), DBError> {
        let query = "INSERT INTO tasks (task_id, task_type, task_state, task_description, created_on) VALUES ($1, $2, $3, $4, $5)";

        sqlx::query(query)
        .bind(task.task_id)
        .bind(task.task_type)
        .bind(task.task_state.to_string())
        .bind(task.task_description)
        .bind(task.created_on)
        .execute(&self.pool)
        .await
        .unwrap();

        Ok(())
    }
}