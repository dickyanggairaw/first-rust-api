use std::{sync::Arc};

use crate::helpers::{hash::{hash_password, verify_password}, jsonwebtoken::generate_token};

use super::{dto::{CreateUserDto, LoginUserDto}, model::User, repository::UserRepository};

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

    pub async fn create_user(&self, dto: CreateUserDto) -> User {
      let hash = hash_password(&dto.password).unwrap();
      self.repository.create_user(dto, hash).await
    }

    pub async fn login_user(&self, dto: LoginUserDto) -> Result<String, &str> {
      let user = self.repository.find_user_by_email(dto.email).await;
      let flag = verify_password(&user.password, &dto.password).unwrap();
      if flag {
        let token = generate_token(&user.email);
        Ok(token.unwrap())
      }else {
        Err("Invalid email or password")
      }
    }
}