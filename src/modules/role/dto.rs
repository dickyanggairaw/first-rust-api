use serde::Deserialize;
use utoipa::ToSchema;


#[derive(Deserialize, ToSchema)]
pub struct CreateRoleDto {
  pub name: String
}