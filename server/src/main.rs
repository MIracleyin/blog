mod article;
mod errors;
mod models;

use article::view;

use errors::CustomError;
use ntex::web::{self, middleware, App, HttpServer};
use sqlx::{Postgres, Pool};
use std::{
    env,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_pool: Pool<Postgres>,
}

#[ntex::main]
async fn main() {
    dotenvy::dotenv().ok();

    env::set_var("RUST_LOG", "ntex=info");
    env_logger::init();

    let db_url = env::var("DATABASE_URL").expect("Please set `DATABASE_URL`");
    println!("{:#?}", db_url);

    // State

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(error)
    })
    .bind("0.0.0.0:12345")
    .unwrap()
    .run()
    .await
    .unwrap();
}

#[web::get("/")]
async fn index() -> String {
    // impl Response
    "Hello, world".into()
}

#[web::get("/error")]
async fn error() -> Result<String, CustomError> {
    Err(CustomError::NotFound("Not found".into()))
}
