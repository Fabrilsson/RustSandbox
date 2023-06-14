use serde::Serialize;
use uuid::Uuid;
use strum_macros::{EnumString, Display};

#[derive(Serialize, Display)]
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
    pub task_id: String,
    pub task_type: String,
    pub task_state: TaskState,
    pub task_description: String,
    pub created_on: String,
    pub completed_on: Option<String>
}

impl Task {
    pub fn new(task_type: String, task_description: String) -> Task {
        Task {
            task_id: Uuid::new_v4().to_string(),
            task_type,
            task_state: TaskState::NotStarted,
            task_description,
            created_on: chrono::offset::Utc::now().to_string(),
            completed_on: None
        }
    }

    pub fn get_id(&self) -> String {
        self.task_id.clone()
    }
}