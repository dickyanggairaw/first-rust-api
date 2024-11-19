use actix_web::{get, web, Responder};

use crate::utils::{message::RESPONSE_GET, response::success_response};

use super::service::UserService;

#[utoipa::path(
  get,
  path = "/api/v1/user/",
  responses(
    (status = 200, description = "Get User")
)
)]
#[get("/")]
pub async fn get_all_user(user_service: web::Data<UserService>) -> impl Responder {
  let users = user_service.get_all().await;
  success_response(users, &RESPONSE_GET)
}