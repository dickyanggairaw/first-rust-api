use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
  pub id: i32,
  pub name: String,
  pub email: String,
  pub password: String,
  pub role_id: i32,
  pub created_at: Option<NaiveDateTime>,
  pub updated_at: Option<NaiveDateTime>,
}