use actix_web::{middleware, web, App, HttpServer};
use confik::{Configuration as _, EnvSource};
use dotenvy::dotenv;
use std::io;
use tokio_postgres::NoTls;

mod core;
mod module;
mod utils;

use crate::core::database::config::ExampleConfig;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    core::logs::main();

    let config = ExampleConfig::builder()
        .override_with(EnvSource::new())
        .try_build()
        .unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    return HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(core::router::routes())
    })
    .bind(config.server_addr.clone())?
    .workers(4)
    .run()
    .await;
}
