use serde::Serialize;

#[derive(Serialize)]
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
    created_on: String,
    completed_on: Option<String>
}

impl Task {
    pub fn new(task_id: String, task_type: String, task_description: String) -> Task {
        Task {
            task_id,
            task_type,
            task_state: TaskState::NotStarted,
            task_description,
            created_on: chrono::offset::Utc::now().to_string(),
            completed_on: None
        }
    }
}