use actix_web::{HttpResponse, post, get};

use crate::utils::response;

#[post("/refresh")]
async fn refresh() -> HttpResponse {
    response::ok("Hello", {})
}

#[post("/logout")]
async fn logout() -> HttpResponse {
    response::ok("Hello", {})
}

#[post("/login")]
pub async fn login() -> HttpResponse {
    response::ok("Hello", {})
}

#[post("/check")]
async fn check() -> HttpResponse {
    response::ok("Hello", {})
}

#[post("/session/init")]
async fn session_init() -> HttpResponse {
    response::ok("Hello", {})
}

#[get("/session/retrieve")]
async fn session_retrieve() -> HttpResponse {
    response::ok("Hello", {})
}