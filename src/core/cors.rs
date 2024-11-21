use actix_cors::Cors;
use actix_web::http::header;

pub fn config() -> Cors {
    return Cors::default()
        .allowed_origin("http://localhost:9000")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .supports_credentials()
        .max_age(3600);
}