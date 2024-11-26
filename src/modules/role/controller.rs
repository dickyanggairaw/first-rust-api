use actix_web::{delete, get, post, put, web, Responder};
use crate::utils::message::{RESPONSE_CREATE, RESPONSE_DELETE, RESPONSE_GET, RESPONSE_UPDATE};
use crate::utils::response::success_response;
use crate::modules::role::RoleService;
use crate::modules::role::dto::CreateRoleDto;

#[utoipa::path(
  get,
  path = "/api/v1/roles/",
  responses(
      (status = 200, description = "Get role")
  ),
  params(
    ("access_token" = String, Header, description = "This is a custom header value"),
  ),
  security(
    ("bearerAuth" = [])
  )
)]
#[get("/")]
pub async fn get_role_handler(role_service: web::Data<RoleService>) -> impl Responder {
  let roles = role_service.get_roles().await;
    success_response(roles, &RESPONSE_GET)
}
#[utoipa::path(
  post,
  path = "/api/v1/roles/",
  responses(
      (status = 200, description = "Create role")
  ),
  request_body = CreateRoleDto,
  params(
    ("access_token" = String, Header, description = "This is a custom header value"),
  )
)]
#[post("/")]
pub async fn create_role_handler (
  role_service: web::Data<RoleService>,
  dto: web::Json<CreateRoleDto>
) -> impl Responder {
  let new_role = role_service.create_role(dto.into_inner()).await;
  success_response(new_role, &RESPONSE_CREATE)
}
#[utoipa::path(
  get,
  path = "/api/v1/roles/{id}",
  responses(
      (status = 200, description = "Get role")
  ),
  params(
    ("id" = i32, Path, description = "Role database id to get Role for"),
    ("access_token" = String, Header, description = "This is a custom header value"),
  )
)]
#[get("/{id}")]
pub async fn get_role_by_id(
  role_service: web::Data<RoleService>,
  path: web::Path<i32>
) -> impl Responder {
  let id = path.into_inner();
  let role = role_service.get_role_by_id(id).await;
  success_response(role, &RESPONSE_GET)
}
#[utoipa::path(
  put,
  path = "/api/v1/roles/{id}",
  responses(
      (status = 200, description = "Get role")
  ),
  params(
    ("id" = i32, Path, description = "Role database id to get Role for"),
    ("access_token" = String, Header, description = "This is a custom header value"),
  ),
  request_body = CreateRoleDto
)]
#[put("/{id}")]
pub async fn update_role(
  role_service: web::Data<RoleService>,
  dto: web::Json<CreateRoleDto>,
  path: web::Path<i32>
) -> impl Responder {
  let id = path.into_inner();
  let role = role_service.update_role(id, dto.into_inner()).await;
  success_response(role, &RESPONSE_UPDATE)
}
#[utoipa::path(
  delete,
  path = "/api/v1/roles/{id}",
  responses(
      (status = 200, description = "Get role", body = String)
  ),
  params(
    ("id" = i32, Path, description = "Role database id to delete Role for"),
    ("access_token" = String, Header, description = "This is a custom header value"),
  )
)]
#[delete("/{id}")]
pub async fn delete_role_by_id(
  role_service: web::Data<RoleService>,
  path: web::Path<i32>
) -> impl Responder {
  let id = path.into_inner();
  let role = role_service.delete_role(id).await;
  success_response(role, &RESPONSE_DELETE)
}