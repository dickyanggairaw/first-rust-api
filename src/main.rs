mod modules;
mod docs;
mod config;
mod helpers;
mod middlewares;
mod utils;
mod schema;

use std::sync::Arc;

use actix_web::{App, HttpServer};
use config::postgresql::establish_connection;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::docs::api_doc::ApiDoc;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let openapi = ApiDoc::openapi();
    let pool = Arc::new(establish_connection().await);
    let address = "127.0.0.1:8080";
    println!("🚀 Starting server at http://{}", address);
    HttpServer::new(move || {
        App::new()
        .service(modules::init_route(pool.clone()))
        .service(
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-doc/openapi.json", openapi.clone())
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
