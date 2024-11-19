use std::sync::Arc;

use actix_web::{web, Scope};
use repository::UserRepository;
use service::UserService;

use crate::config::postgresql::DbPool;
pub mod repository;
pub mod service;
pub mod model;
pub mod controller;

pub fn user_routes(pool: Arc<DbPool>) -> Scope {
  let user_repository = Arc::new(UserRepository::new(pool.clone()));
  let user_service = UserService::new(user_repository);
  web::scope("user")
    .app_data(web::Data::new(user_service))
    .service(controller::get_all_user)
}