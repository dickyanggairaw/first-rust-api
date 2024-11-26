use std::rc::Rc;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::InternalError;
use actix_web::{Error, HttpMessage, HttpResponse};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use crate::helpers::jsonwebtoken::verify_jwt;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
  where 
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
      ok(AuthMiddlewareMiddleware {
        service: Rc::new(service),
    })
    }
}

pub struct AuthMiddlewareMiddleware<S> {
  service: Rc<S>,
}

impl <S, B> Service<ServiceRequest> for AuthMiddlewareMiddleware<S>
where 
  S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
  B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, req: ServiceRequest) -> Self::Future {
      let auth_header = req.headers().get("access_token").and_then(|h| h.to_str().ok());
      let o = auth_header.clone().unwrap().to_string();
      let token_aja = verify_jwt(&o).ok();
      if let Some(data) = token_aja {
          req.extensions_mut().insert(data.claims);
          let fut = self.service.call(req);
          Box::pin(async move {
            let res = fut.await?;
            Ok(res)
          })
      } else {
        let response = HttpResponse::Unauthorized().body("Invalid or missing JWT");
        let error = InternalError::from_response("Authentication Error", response).into();
        Box::pin(async { Err(error) })
      }
  }

  forward_ready!(service);
}