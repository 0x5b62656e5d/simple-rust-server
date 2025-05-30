use actix_web::{App, HttpServer};

mod routes;
use routes::hello::hello;

mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("0.0.0.0", 8300))?
        .run()
        .await
}
