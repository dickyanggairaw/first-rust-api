use actix_web::{get, web, Responder, post};
use crate::utils::{message::{RESPONSE_CREATE, RESPONSE_GET}, response::success_response};
use super::{dto::{CreateUserDto, LoginUserDto}, service::UserService};

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
#[utoipa::path(
  post,
  path = "/api/v1/user/",
  responses(
      (status = 200, description = "Create user")
  ),
  request_body = CreateUserDto
)]
#[post("/")]
pub async fn register(
  user_service: web::Data<UserService>,
  dto: web::Json<CreateUserDto>
) -> impl Responder {
  let new_user = user_service.create_user(dto.into_inner()).await;
  success_response(new_user, &RESPONSE_CREATE)
}
#[utoipa::path(
  post,
  path = "/api/v1/user/login",
  responses(
      (status = 200, description = "login user")
  ),
  request_body = LoginUserDto
)]
#[post("/login")]
pub async fn login(
  user_service: web::Data<UserService>,
  dto: web::Json<LoginUserDto>
) -> impl Responder {
  let new_user = user_service.login_user(dto.into_inner()).await;
  success_response(new_user.unwrap(), &RESPONSE_CREATE)
}