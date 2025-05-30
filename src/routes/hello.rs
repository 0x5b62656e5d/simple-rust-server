use actix_web::{HttpResponse, Responder, get};
use crate::util::standard_response::StandardResponse;

#[get("/")]
async fn hello() -> impl Responder {
    let response: StandardResponse<()> = StandardResponse {
        success: true,
        message: "Hello, world!".to_string(),
        data: None,
        error: None,
    };

    HttpResponse::Ok().json(response)
}
