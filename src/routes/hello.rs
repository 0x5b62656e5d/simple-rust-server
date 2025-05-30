use actix_web::{HttpResponse, Responder, get};
use serde::Serialize;

#[derive(Serialize)]
struct HelloResponse {
    success: bool,
    message: String,
}


#[get("/")]
async fn hello() -> impl Responder {
    let response = HelloResponse {
        success: true,
        message: "Hello, world!".to_string(),
    };

    HttpResponse::Ok().json(response)
}
