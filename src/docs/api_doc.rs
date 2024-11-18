use utoipa::OpenApi;

use crate::modules::role::dto::CreateRoleDto;
use crate::modules::role::controller::__path_get_role_handler;
use crate::modules::role::controller::{__path_create_role_handler, __path_get_role_by_id, __path_update_role, __path_delete_role_by_id};

#[derive(OpenApi)]
#[openapi(paths(get_role_handler, create_role_handler, get_role_by_id, update_role, delete_role_by_id), components(schemas(CreateRoleDto)))]
pub struct ApiDoc;