use std::sync::{Arc, RwLock};
use sqlx::query_as;
use crate::{config::postgresql::DbPool, helpers::redis::RedisHelper};
use super::{dto::CreateRoleDto, model::Role};

pub struct RoleRepository {
  pool: Arc<DbPool>,
  redis: Arc<RwLock<RedisHelper>>
}

const KEY_ROLE: &'static str = "role";

impl RoleRepository {
  pub fn new (pool: Arc<DbPool>, redis: Arc<RwLock<RedisHelper>>) -> Self {
    Self {pool, redis}
  }
    pub async fn get_all(&mut self) -> Vec<Role> {
      let mut redis = self.redis.write().unwrap();
      let redis_result = redis.get_value(KEY_ROLE).expect("msg");
      if redis_result.is_some() {
        println!("masuk kondisi redis");
        serde_json::from_str::<Vec<Role>>(&redis_result.clone().unwrap().to_string()).unwrap()
      }else {
        let result = query_as::<_, Role>("SELECT * FROM role")
          .fetch_all(&*self.pool)
          .await
          .expect("Failed to fetch users");
        let result_string= serde_json::to_string(&result).unwrap();
        let _ = redis.set_value(KEY_ROLE, &result_string).unwrap();
        result
      }
    }
    pub async fn create(&self, dto: CreateRoleDto) -> Role {
      let result = query_as::<_, Role>("insert into role (role_name) values ($1) returning *").bind(dto.name)
      .fetch_one(&*self.pool)
      .await
      .expect("Failed to fetch users");
      let mut redis = self.redis.write().unwrap();
      let _ = redis.del_value(KEY_ROLE).unwrap();
      result
    }
    pub async fn get_role_by_id(&self, id: i32) -> Role {
      query_as::<_,Role>("select * from role where role_id = $1").bind(id).fetch_one(&*self.pool).await.expect("Failed to fetch role")
    }
    pub async fn update_role(&self, id: i32, dto: CreateRoleDto) -> Role {
      query_as::<_,Role>("update role set role_name = $1 where role_id = $2 returning *").bind(dto.name).bind(id).fetch_one(&*self.pool).await.expect("Failed to fetch role")
    }
    pub async fn delete_role(&self, id: i32) -> String {
      println!("{}",id);
      String::from("Successfully delete role : {}")
    }
}