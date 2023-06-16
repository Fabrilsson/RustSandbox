use actix_web::{
    get,
    post,
    put,
    error::ResponseError,
    web::{Path, Json, Data, Query},
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
pub struct Paging {
    skip: i32,
    take: i32
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

#[get("/tasks")]
pub async fn get_tasks(paging: Query<Paging>, repository: Data<TaskRepository>) -> Json<Vec<Task>> {
    let page = paging.into_inner();

    let tasks = repository.get_tasks(&page.skip, &page.take).await.unwrap();

    Json(tasks)
}

#[get("/task/{task_id}")]
pub async fn get_task(id: Path<TaskIdentifier>, repository: Data<TaskRepository>) -> Json<Task> {
    let task = repository.get_task(&id.into_inner().task_id).await.unwrap();

    Json(task)
}

#[post("/task")]
pub async fn post_task(repository: Data<TaskRepository>, task_request: Json<TaskPostRequest>) -> Result<Json<TaskIdentifier>, TaskError> {

    let task = Task::new(task_request.task_type.clone(), task_request.task_description.clone());
    let identifier = task.get_id();

    match repository.insert_task(task).await {
        Ok(()) => Ok(Json(TaskIdentifier { task_id: String::from(identifier) })),
        Err(_) => Err(TaskError::TaskCreationFailure)
    }
}