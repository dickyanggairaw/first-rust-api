use super::{dto::CreateRoleDto, model::Role};

pub struct RoleRepository;

impl RoleRepository {
    pub async fn get_all(&self) -> Vec<Role> {
      let result = vec![
      Role{
        id: 1,
        name: String::from("admin"),
      },
      Role{
        id: 2,
        name: String::from("user"),
      }];

    result
    }
    pub async fn create(&self, dto: CreateRoleDto) -> Role {
      Role {id: 3, name: dto.name}
    }
    pub async fn get_role_by_id(&self, id: u64) -> Role {
      Role {id, name: String::from("admin")}
    }
    pub async fn update_role(&self, id: u64, dto: CreateRoleDto) -> Role {
      Role {id, name: dto.name}
    }
    pub async fn delete_role(&self, id: u64) -> String {
      println!("{}",id);
      String::from("Successfully delete role : {}")
    }
}