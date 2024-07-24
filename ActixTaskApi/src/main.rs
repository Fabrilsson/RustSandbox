mod controllers;
mod model;
mod repositories;
mod database;

use actix_web::{HttpServer, App, web::Data, middleware::Logger};
use repositories::task_repository::{TaskRepository};

use controllers::task_controller::{
    get_tasks,
    get_task,
    post_task
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    let repository: TaskRepository = TaskRepository::new().await;

    repository.migrate().await.unwrap();

    HttpServer::new(move || {

        let logger = Logger::default();

        let data = Data::new(repository.clone());

        App::new()
        .app_data(data)
        .wrap(logger)
        .service(get_tasks)
        .service(get_task)
        .service(post_task)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}