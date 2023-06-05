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

#[derive(Serialize, Deserialize)]
pub struct TaskIdentifier {
    glogal_id: String
}

#[get("/task/{glogal_id}")]
pub async fn get_task(id: Path<TaskIdentifier>) -> Json<String> {
    Json(id.into_inner().glogal_id)
}