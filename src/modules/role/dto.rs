use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(ToSchema, Serialize, Deserialize)]
pub struct CreateRoleDto {
  pub name: String
}