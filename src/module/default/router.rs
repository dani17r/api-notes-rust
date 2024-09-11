use actix_web::{web, Scope};
use crate::module::default::controller;

pub fn routes() -> Scope {
    return web::scope("")
        .route("", web::get().to(controller::index))
}