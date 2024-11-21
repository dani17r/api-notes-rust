use crate::module::tags::controller;
use actix_web::{web, Scope};

pub fn routes() -> Scope {
    return web::scope("/tag")
        .route("all", web::get().to(controller::get_many_tags))
        .route("one/{id}", web::get().to(controller::get_one_tag))
        .route("update", web::put().to(controller::update_one_tag))
        .route("create", web::post().to(controller::create_one_tag))
        .route("delete", web::delete().to(controller::delete_many_tags));
}
