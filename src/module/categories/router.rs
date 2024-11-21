use crate::module::categories::controller;
use actix_web::{web, Scope};

pub fn routes() -> Scope {
    return web::scope("/category")
        .route("all", web::get().to(controller::get_many_categories))
        .route("one/{id}", web::get().to(controller::get_one_category))
        .route("update", web::put().to(controller::update_one_category))
        .route("create", web::post().to(controller::create_one_category))
        .route("delete", web::delete().to(controller::delete_many_categories));
}
