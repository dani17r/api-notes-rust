use actix_web::{web, Scope};

use crate::module;

pub fn routes() -> Scope {
    return web::scope("")
        .service(
            web::scope("/api")
            .service(module::tags::router::routes())
            .service(module::notes::router::routes())
            .service(module::categories::router::routes())
            .service(module::default::router::routes())
        )    
        .route("/", web::get().to(module::default::controller::empty))
}