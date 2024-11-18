use std::sync::Arc;

use super::{dto::CreateRoleDto, model::Role, repository::RoleRepository};

pub struct RoleService {
  repository: Arc<RoleRepository>,
}

impl RoleService {
    pub fn new(repository: Arc<RoleRepository>) -> Self {
      Self {repository}
    }

    pub async fn get_roles(&self) -> Vec<Role> {
      self.repository.get_all().await
    }

    pub async fn create_role(&self, dto: CreateRoleDto) -> Role {
      self.repository.create(dto).await
    }
    pub async fn get_role_by_id(&self, id: u64) -> Role {
      self.repository.get_role_by_id(id).await
    }
    pub async fn update_role(&self, id: u64, dto: CreateRoleDto) -> Role {
      self.repository.update_role(id, dto).await
    }
    pub async fn delete_role(&self, id: u64) -> String {
      self.repository.delete_role(id).await
    }
}