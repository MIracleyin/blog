use ntex::web::{self, middleware, App, HttpRequest, HttpResponse, HttpServer};
use std::env;

#[ntex::main]
async fn main() {
    env::set_var("RUST_LOG", "ntex=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
    })
    .bind("0.0.0.0:12345")
    .unwrap()
    .run()
    .await
    .unwrap();
}

#[web::get("/")]
async fn index() -> String { // impl Response
    "Hello, world".into()
}
