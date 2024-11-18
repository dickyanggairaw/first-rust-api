pub mod controller;
pub mod model;
pub mod dto;
pub mod repository;
pub mod service;
use std::sync::Arc;

use actix_web::{web, Scope};
use service::RoleService;
use crate::modules::role::repository::RoleRepository;

pub fn role_routes() -> Scope {
  let role_repository = Arc::new(RoleRepository);
  let role_service = RoleService::new(role_repository);
  web::scope("/roles")
      .app_data(web::Data::new(role_service))
      .service(controller::get_role_handler)
      .service(controller::create_role_handler)
      .service(controller::get_role_by_id)
      .service(controller::update_role)
      .service(controller::delete_role_by_id)
}