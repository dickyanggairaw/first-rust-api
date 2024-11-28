use std::sync::{Arc, RwLock};

use actix_web::web;
use repository::UserRepository;
use service::UserService;

use crate::{config::postgresql::DbPool, helpers::redis::RedisHelper, middlewares::auth::AuthMiddleware};
pub mod repository;
pub mod service;
pub mod model;
pub mod controller;
pub mod dto;

pub fn user_routes(pool: Arc<DbPool>, redis: Arc<RwLock<RedisHelper>>) -> impl actix_web::dev::HttpServiceFactory {
  let user_repository = Arc::new(UserRepository::new(pool.clone(), redis.clone()));
  let user_service = UserService::new(user_repository);
  web::scope("user")
    .app_data(web::Data::new(user_service))
    .service(controller::register)
    .service(controller::login)
    .service(
      web::scope("/list")
        .wrap(AuthMiddleware)
        .service(controller::get_all_user)
    )
}