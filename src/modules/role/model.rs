use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Role {
    pub role_id: i32,
    pub role_name: String,
    pub description: Option<String>,
    // #[schema(example = "2024-11-11T12:34:56")]
    pub created_at: Option<NaiveDateTime>,
    // #[schema(example = "2024-11-11T12:34:56")]
    pub updated_at: Option<NaiveDateTime>,
}