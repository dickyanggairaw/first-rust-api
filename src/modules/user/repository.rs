use std::sync::{Arc, RwLock};
use sqlx::query_as;
use crate::{config::postgresql::DbPool, helpers::redis::RedisHelper};
use super::{dto::CreateUserDto, model::User};

#[allow(dead_code)]
pub struct UserRepository {
  pool: Arc<DbPool>,
  redis: Arc<RwLock<RedisHelper>>
}

impl UserRepository {
    pub fn new(pool: Arc<DbPool>, redis: Arc<RwLock<RedisHelper>>) ->Self {
      Self {pool, redis}
    }

    pub async fn fetch_all(&self) -> Vec<User> {
      query_as::<_,User>("select * from users")
        .fetch_all(&*self.pool)
        .await
        .expect("cannot get user")
    }

    pub async fn create_user(&self,dto: CreateUserDto, hash: String) -> User {
      query_as::<_,User>("INSERT INTO users (name, email, password, role_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, NOW(), NOW())
        RETURNING *
      ").bind(dto.name).bind(dto.email).bind(hash).bind(dto.role_id)
      .fetch_one(&*self.pool)
      .await
      .expect("Failed create user")
    }

    pub async  fn find_user_by_email(&self, email: String) -> User {
      query_as::<_,User>("select * from users where email = $1")
        .bind(email)
        .fetch_one(&*self.pool)
        .await
        .expect("cannot get user")
    }
}