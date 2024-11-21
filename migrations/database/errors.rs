use actix_web::{HttpResponse, ResponseError};
use derive_more::{Display, Error, From};

#[warn(dead_code)]
#[derive(Debug, Display, Error, From)]
pub enum MyError {
    PGError(tokio_postgres::Error),
    PGMError(tokio_pg_mapper::Error),
    PoolError(deadpool_postgres::PoolError),
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            MyError::PoolError(ref err) => {
                HttpResponse::InternalServerError().body(err.to_string())
            }
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
