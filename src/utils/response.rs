use actix_web::HttpResponse;
use serde::Serialize;
use crate::service::models::response;

pub fn not_found<T: Serialize>(message: String, json: T) -> HttpResponse {
    return HttpResponse::NotFound().json(response::Response {
        code: 404,
        status: "NOT_FOUND".into(),
        message: message,
        content: json
    });
}

pub fn unauthorized<T: Serialize>(message: String, json: T) -> HttpResponse {
    return HttpResponse::Unauthorized().json(response::Response {
        code: 401,
        status: "UNAUTHORIZED".into(),
        message: message,
        content: json
    });
}

pub fn ok<T: Serialize, S: ToString>(message: S, json: T) -> HttpResponse {
    return HttpResponse::Ok().json(response::Response {
        code: 200,
        status: "OK".into(),
        message: message.to_string(),
        content: json
    });
}

pub fn internal_server_error<T: Serialize>(message: String, json: T) -> HttpResponse {
    return HttpResponse::InternalServerError().json(response::Response {
        code: 500,
        status: "INTERNAL_SERVER_ERROR".into(),
        message: message,
        content: json
    });
}

pub fn bad_request<T: Serialize>(message: String, json: T) -> HttpResponse {
    return HttpResponse::BadRequest().json(response::Response {
        code: 400,
        status: "BAD_REQUEST".into(),
        message: message,
        content: json
    });
}