use std::sync::Arc;

use sqlx::query_as;

use crate::config::postgresql::DbPool;

use super::model::User;

pub struct UserRepository {
  pool: Arc<DbPool>,
}

impl UserRepository {
    pub fn new(pool: Arc<DbPool>) ->Self {
      Self {pool}
    }

    pub async fn fetch_all(&self) -> Vec<User> {
      query_as::<_,User>("select * from users")
        .fetch_all(&*self.pool)
        .await
        .expect("cannot get user")
    }
}