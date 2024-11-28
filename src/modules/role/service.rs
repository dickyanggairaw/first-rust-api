use std::sync::{Arc, RwLock};

use super::{dto::CreateRoleDto, model::Role, repository::RoleRepository};

pub struct RoleService {
  repository: Arc<RwLock<RoleRepository>>,
}

impl RoleService {
    pub fn new(repository: Arc<RwLock<RoleRepository>>) -> Self {
      Self {repository}
    }

    pub async fn get_roles(&self) -> Vec<Role> {
      self.repository.write().unwrap().get_all().await
    }

    pub async fn create_role(&self, dto: CreateRoleDto) -> Role {
      self.repository.write().unwrap().create(dto).await
    }
    pub async fn get_role_by_id(&self, id: i32) -> Role {
      self.repository.write().unwrap().get_role_by_id(id).await
    }
    pub async fn update_role(&self, id: i32, dto: CreateRoleDto) -> Role {
      self.repository.write().unwrap().update_role(id, dto).await
    }
    pub async fn delete_role(&self, id: i32) -> String {
      self.repository.write().unwrap().delete_role(id).await
    }
}