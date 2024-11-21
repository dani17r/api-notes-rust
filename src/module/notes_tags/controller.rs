use crate::{
    core::database::errors::MyError,
    module::notes::models::AddTagsInNote,
};
use actix_web::{web, HttpResponse};
use deadpool_postgres::{Client, GenericClient, Pool};
use tokio_pg_mapper::FromTokioPostgresRow;

use super::models::NoteTags;

//--------------------------------------------------------------

/* POST */
#[warn(dead_code)]
pub async fn add_tags_in_note(
    body: web::Json<AddTagsInNote>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let body_params: AddTagsInNote = body.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let stmt = include_str!("../notes_tags/querys/add_tags_in_note.sql");
    let stmt = client.prepare(&stmt).await.unwrap();

    let result = client
        .query(&stmt, &[&body_params.note_id, &body_params.tag_ids])
        .await?
        .iter()
        .map(|row| NoteTags::from_row_ref(row).unwrap())
        .collect::<Vec<NoteTags>>();

     if result.is_empty() {
        return Err(MyError::NotFound);
    }

    Ok(HttpResponse::Ok().json(result))
}

/* POST */
// #[warn(dead_code)]
// pub async fn update_tags_in_note(
//     body: web::Json<AddTagsInNote>,
//     db_pool: web::Data<Pool>,
// ) -> Result<HttpResponse, MyError> {
//     let body_params: AddTagsInNote = body.into_inner();

//     let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

//     let stmt = include_str!("../notes_tags/querys/update_tags_in_note.sql");
//     let stmt = client.prepare(&stmt).await.unwrap();

//     let result = client
//         .query(&stmt, &[&body_params.note_id, &body_params.tag_ids])
//         .await?
//         .iter()
//         .map(|row| NoteTags::from_row_ref(row).unwrap())
//         .collect::<Vec<NoteTags>>();

//     if result.is_empty() {
//         println!("--{:?}--", result);
//         return Err(MyError::NotFound);
//     }

//     Ok(HttpResponse::Ok().json(result))
// }

/* POST */
#[warn(dead_code)]
pub async fn delete_tags_in_note(
    body: web::Json<AddTagsInNote>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let body_params: AddTagsInNote = body.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let stmt = include_str!("../notes_tags/querys/delete_tags_in_note.sql");
    let stmt = client.prepare(&stmt).await.unwrap();

    let result = client
        .query(&stmt, &[&body_params.note_id, &body_params.tag_ids])
        .await?
        .iter()
        .map(|row| NoteTags::from_row_ref(row).unwrap())
        .collect::<Vec<NoteTags>>();

    if result.is_empty() {
        return Err(MyError::NotFound);
    }

   Ok(HttpResponse::Ok().json(result))
}
