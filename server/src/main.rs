mod article;
mod comment;
mod errors;
mod models;
mod user;
// mod middlewares;

use errors::CustomError;
use ntex::web::{self, middleware, service, App, HttpServer};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{env, sync::Arc};

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
    // println!("{:#?}", db_url);

    // State
    let app_state = Arc::new(AppState {
        db_pool: PgPoolOptions::new()
            .max_connections(10)
            .connect(&db_url)
            .await
            .unwrap(),
    });

    HttpServer::new(move || {
        App::new()
            .state(Arc::clone(&app_state))
            .wrap(middleware::Logger::default())
            .configure(route)
    })
    .bind("0.0.0.0:12345")
    .unwrap()
    .run()
    .await
    .unwrap();
}

fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/article")
            .route("/{id}", web::get().to(article::view::get_article))
            .route("", web::post().to(article::new::new_article))
            .route("", web::put().to(article::edit::edit_article))
            .route("/{id}", web::delete().to(article::delete::delete_article))
            .route(
                "/search/{keyword}",
                web::get().to(article::search::search_article),
            ),
    )
    .service(web::scope("/articles").route("", web::get().to(article::view::get_articles_preview)))
    .service(web::scope("/user").route("/login", web::post().to(user::login::github_login)))
    .service(
        web::scope("/comment")
            .route(
                "/{id}",
                web::get().to(comment::view::get_comments_for_article),
            )
            .route("", web::post().to(comment::new::new_comment)),
    );
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
