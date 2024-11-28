pub mod role;
pub mod user;

use std::sync::{Arc, RwLock};

use actix_web::{web, Scope};
use crate::{config::postgresql::DbPool, helpers::redis::RedisHelper};

pub fn init_route(pool: Arc<DbPool>, redis: Arc<RwLock<RedisHelper>>) -> Scope {
  let v1 = "/api/v1";
  web::scope(v1)
    .service(user::user_routes(pool.clone(), redis.clone()))
    .service(role::role_routes(pool.clone(), redis.clone()))
}