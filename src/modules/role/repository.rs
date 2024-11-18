use sqlx::query_as;

use crate::config::postgresql::establish_connection;

use super::{dto::CreateRoleDto, model::Role};

pub struct RoleRepository;

impl RoleRepository {
    pub async fn get_all(&self) -> Vec<Role> {
      let pool = establish_connection().await;
      query_as::<_, Role>("SELECT * FROM role")
        .fetch_all(&pool)
        .await
        .expect("Failed to fetch users")
    }
    pub async fn create(&self, dto: CreateRoleDto) -> Role {
      let pool = establish_connection().await;
      query_as::<_, Role>("insert into role (role_name) values ($1) returning *").bind(dto.name)
      .fetch_one(&pool)
      .await
      .expect("Failed to fetch users")
    }
    pub async fn get_role_by_id(&self, id: i32) -> Role {
      let pool = establish_connection().await;
      query_as::<_,Role>("select * from role where role_id = $1").bind(id).fetch_one(&pool).await.expect("Failed to fetch role")
    }
    pub async fn update_role(&self, id: i32, dto: CreateRoleDto) -> Role {
      let pool = establish_connection().await;
      query_as::<_,Role>("update role set role_name = $1 where role_id = $2 returning *").bind(dto.name).bind(id).fetch_one(&pool).await.expect("Failed to fetch role")
    }
    pub async fn delete_role(&self, id: i32) -> String {
      println!("{}",id);
      String::from("Successfully delete role : {}")
    }
}