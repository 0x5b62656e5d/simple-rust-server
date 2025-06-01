use crate::util::standard_response::StandardResponse;
use actix_web::{HttpResponse, Responder, get, web::Json};
use serde_json::{Value as JsonValue, json};

#[get("/coffee")]
async fn coffee() -> impl Responder {
    let coffee = json!({
        "teapot": "Oops! I can't brew coffee ;c".to_string(),
        "text": "Beep boop".to_string(),
    });

    let response: StandardResponse<JsonValue> = StandardResponse {
        success: false,
        message: "Sleepy...".to_string(),
        data: Some(Json(coffee)),
        error: Some("I'm a teapot!".to_string()),
    };

    HttpResponse::ImATeapot().json(response)
}
