use actix_web::{web, Responder};

use crate::service::utils::response;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
    web::resource("/test")  
        .route(web::get().to(manual_hello))
    );
}

async fn manual_hello() -> impl Responder {
    response::ok("Hello".into(), {})
}