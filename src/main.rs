use crate::routes::init_routes;

use actix_web::{App, HttpServer};

mod routes;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(init_routes))
        .bind(("0.0.0.0", 8300))?
        .run()
        .await
}
