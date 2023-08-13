use actix_web::web;

use super::controller as ctrl;

pub fn register(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/authentication")
          .service(ctrl::login)
          .service(ctrl::refresh)
          .service(ctrl::check)
          .service(ctrl::logout)
      );
}