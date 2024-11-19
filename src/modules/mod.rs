pub mod role;
pub mod user;

use std::sync::Arc;

use actix_web::{web, Scope};
use crate::config::postgresql::DbPool;

pub fn init_route(pool: Arc<DbPool>) -> Scope {
  let v1 = "/api/v1";
  web::scope(v1)
    .service(role::role_routes(pool.clone()))
    .service(user::user_routes(pool))
}