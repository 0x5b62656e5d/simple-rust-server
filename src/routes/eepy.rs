use crate::util::standard_response::StandardResponse;
use actix_web::{HttpResponse, Responder, get, web::Json};
use serde_json::{Value as JsonValue, json};

#[get("/eepy")]
async fn eepy() -> impl Responder {
    let tea = json!( {
        "tea_type": "Matcha latte time c:".to_string(),
        "text": "Beep boop".to_string(),
    });

    let response: StandardResponse<JsonValue> = StandardResponse {
        success: false,
        message: "Brewing some tea...".to_string(),
        data: Some(Json(tea)),
        error: Some("I'm eepy".to_string()),
    };

    HttpResponse::ImATeapot().json(response)
}
