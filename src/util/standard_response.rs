use actix_web::web::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct StandardResponse<T> {
    pub success: bool,
    pub message: String,
    pub data: Option<Json<T>>,
    pub error: Option<String>,
}
