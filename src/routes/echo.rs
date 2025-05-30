use crate::util::standard_response::StandardResponse;
use actix_web::{HttpResponse, Responder, post, web::Json};
use serde::{Deserialize};
use serde_json::Value as JsonValue;

#[derive(Deserialize)]
struct Body {
    string: JsonValue,
}

#[post("/echo")]
async fn echo(body: Option<Json<Body>>) -> impl Responder {
    let Some(body) = body else {
        let response: StandardResponse<String> = StandardResponse {
            success: false,
            message: "No body provided".to_string(),
            data: None,
            error: Some("Body is missing".to_string()),
        };

        return HttpResponse::BadRequest().json(response);
    };

    let parsed_value = match &body.string {
        JsonValue::String(s) => s.clone(),
        _ => (&body.string).to_string(),
    };

    if parsed_value.trim().is_empty() || parsed_value == "null" {
        let response: StandardResponse<String> = StandardResponse {
            success: false,
            message: "String cannot be empty".to_string(),
            data: None,
            error: Some("Empty string provided".to_string()),
        };

        return HttpResponse::BadRequest().json(response);
    }

    let response: StandardResponse<String> = StandardResponse {
        success: true,
        message: "Echo!".to_string(),
        data: Some(Json(parsed_value)),
        error: None,
    };

    HttpResponse::Ok().json(response)
}
