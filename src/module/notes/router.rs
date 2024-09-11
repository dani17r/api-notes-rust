use crate::module::notes::controller;
use actix_web::{web, Scope};

pub fn routes() -> Scope {
    return web::scope("/note")
        .route("all", web::get().to(controller::get_many_notes))
        .route("one/{id}", web::get().to(controller::get_one_note))
        .route("update", web::put().to(controller::update_one_note))
        .route("create", web::post().to(controller::create_one_note))
        .route("delete", web::delete().to(controller::delete_many_notes));
}
