use actix_web::{
    get,
    post,
    put,
    error::ResponseError,
    web::{Path, Json, Data},
    HttpResponse,
    http::{header::ContentType, StatusCode}
};

use serde::{Serialize, Deserialize};
use derive_more::Display;

use crate::{repositories::task_repository::TaskRepository, model::task::{Task, TaskState}};

#[derive(Serialize, Deserialize)]
pub struct TaskIdentifier {
    task_id: String
}

#[derive(Deserialize)]
pub struct TaskPostRequest {
    task_type: String,
    task_description: String
}

#[derive(Debug, Display)]
pub enum TaskError {
    TaskNotFound,
    TaskUpdateFailure,
    TaskCreationFailure,
    BadTaskRequest
}

impl ResponseError for TaskError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            TaskError::TaskNotFound => StatusCode::NOT_FOUND,
            TaskError::TaskUpdateFailure => StatusCode::FAILED_DEPENDENCY,
            TaskError::TaskCreationFailure => StatusCode::FAILED_DEPENDENCY,
            TaskError::BadTaskRequest => StatusCode::BAD_REQUEST
        }
    }
}

#[get("/task/{task_id}")]
pub async fn get_task(id: Path<TaskIdentifier>, repository: Data<TaskRepository>) -> Json<String> {
    Json(id.into_inner().task_id)
}

#[post("/task")]
pub async fn post_task(repository: Data<TaskRepository>, task: Json<TaskPostRequest>) -> Result<Json<TaskIdentifier>, TaskError> {

    let task = Task::new(task.task_type.clone(), task.task_description.clone());
    let identifier = task.get_id();

    match repository.insert_task(task).await {
        Ok(()) => Ok(Json(TaskIdentifier { task_id: String::from(identifier) })),
        Err(_) => Err(TaskError::TaskCreationFailure)
    }
}