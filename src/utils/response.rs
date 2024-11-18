use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T>{
  pub success: bool,
  pub message: String,
  pub data: Option<T>
}
pub fn success_response<T: Serialize>(data: T, message: &str) -> HttpResponse {
  let response = ApiResponse {
    success: true,
    message: message.to_string(),
    data: Some(data),
  };
  HttpResponse::Ok().json(response)
}
#[allow(dead_code)]
pub fn error_response<T: Serialize>(message: &str, status: StatusCode) -> HttpResponse {
  let response: ApiResponse<T> = ApiResponse {
    success: false,
    message: message.to_string(),
    data: None,
  };
  HttpResponse::build(status).json(response)
}