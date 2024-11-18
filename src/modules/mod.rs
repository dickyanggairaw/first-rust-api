pub mod role;

use actix_web::{web, Scope};

pub fn init_route() -> Scope {
  let v1 = "/api/v1";
  web::scope(v1)
    .service(role::role_routes())
}