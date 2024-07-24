use std::collections::binary_heap;
use std::str::FromStr;

use crate::database::db::{DBError, create_pool};
use crate::model::task::{Task, TaskState};

use sqlx::{PgPool, Row};
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

    pub async fn get_tasks(&self, skip: &i32, take: &i32) -> Result<Vec<Task>, DBError> {
        let query = "SELECT * FROM tasks LIMIT $1 OFFSET $2";

        let rows = sqlx::query(query)
        .bind(take)
        .bind(skip)
        .fetch_all(&self.pool)
        .await
        .unwrap();

        let tasks = rows.iter().map(|row| {
            Task {
                task_id: row.get("task_id"),
                task_type: row.get("task_type"),
                task_state: TaskState::from_str(row.get("task_state")).unwrap(),
                task_description: row.get("task_description"),
                created_on: row.get("created_on"),
                completed_on: row.get("completed_on"),
            }
        }).collect();

        Ok(tasks)
    }

    pub async fn get_task(&self, task_id: &String) -> Result<Task, DBError> {
        let query = "SELECT * FROM tasks WHERE task_id = $1";

        let row = sqlx::query(query)
        .bind(task_id)
        .fetch_one(&self.pool)
        .await
        .unwrap();

        let task = Task {
            task_id: row.get("task_id"),
            task_type: row.get("task_type"),
            task_state: TaskState::from_str(row.get("task_state")).unwrap(),
            task_description: row.get("task_description"),
            created_on: row.get("created_on"),
            completed_on: row.get("completed_on"),
        };

        Ok(task)
    }

    pub async fn insert_task<'s>(&'s self, task: Task) -> Result<Task, DBError> {
        let query = "INSERT INTO tasks (task_id, task_type, task_state, task_description, created_on) VALUES ($1, $2, $3, $4, $5)";

        sqlx::query(query)
        .bind(&task.task_id)
        .bind(&task.task_type)
        .bind(&task.task_state.to_string())
        .bind(&task.task_description)
        .bind(&task.created_on)
        .execute(&self.pool)
        .await
        .unwrap();

        Ok(task)
    }

}