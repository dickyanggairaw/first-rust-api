use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateUserDto {
  pub name: String,
  pub email: String,
  pub password: String,
  pub role_id: i32,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginUserDto {
  pub email: String,
  pub password: String,
}