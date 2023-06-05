mod controllers;
mod model;
mod repositories;
mod database;

use actix_web::{HttpServer, App, web::Data, middleware::Logger};
use std::error::Error;
use sqlx::{PgPool};

use controllers::task_controller::{
    get_task
};
use database::db::{
    create_pool
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    let pool: PgPool = create_pool().await.expect("Failed to create pool");

    sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to execute migrations");

    HttpServer::new(move || {

        let logger = Logger::default();

        App::new()
        .app_data(pool.clone())
        .wrap(logger)
        .service(get_task)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}