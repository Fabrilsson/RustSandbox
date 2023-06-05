use serde::Serialize;
use chrono::{DateTime};

pub enum TaskState {
    NotStarted,
    InProgress,
    Completed,
    Paused,
    Failed,
    Deleted
}

#[derive(Serialize)]
pub struct Task {
    task_id: String,
    task_type: String,
    task_state: TaskState,
    task_description: String,
    created_on: DateTime,
    completed_on: Option<DateTime>
}

impl Task {
    pub fn new(task_id: String, task_type: String, task_description: String) -> Task {
        Task {
            task_id,
            task_type,
            task_state: TaskState::NotStarted,
            task_description,
            created_on: chrono::offset::Utc::now(),
            completed_on: Option
        }
    }
}