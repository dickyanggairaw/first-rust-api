use std::sync::Arc;

use super::{model::User, repository::UserRepository};

pub struct UserService {
  repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<UserRepository>) -> Self {
      Self {repository}
    }

    pub async fn get_all(&self) -> Vec<User> {
      self.repository.fetch_all().await
    }
}