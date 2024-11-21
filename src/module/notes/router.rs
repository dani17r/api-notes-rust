use crate::module::{notes, notes_tags};
use actix_web::{web, Scope};

pub fn routes() -> Scope {
    return web::scope("/note")
        .route("all", web::get().to(notes::controller::get_many_notes))
        .route("one/{id}", web::get().to(notes::controller::get_one_note))
        .route("update", web::put().to(notes::controller::update_one_note))
        .route("create", web::post().to(notes::controller::create_one_note))
        .route("add/tags", web::post().to(notes_tags::controller::add_tags_in_note))
        // .route("update/tags", web::post().to(notes_tags::controller::update_tags_in_note))
        .route("delete/tags", web::post().to(notes_tags::controller::delete_tags_in_note))
        .route("delete", web::delete().to(notes::controller::delete_many_notes));
}
