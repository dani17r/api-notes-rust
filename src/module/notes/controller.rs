use crate::{
    core::database::errors::MyError,
    module::{
        default::{
            models::GetResponseParams,
            types::{QuerysParams, VecJson},
        },
        notes::models::{Note, NoteUseCreate, NoteUseUpdate},
    },
    utils::querys::{get_fields, get_pagination, get_response, get_search, get_sort},
};
use actix_web::{web, HttpResponse};
use deadpool_postgres::{Client, GenericClient, Pool};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::types::ToSql;

use super::models::Ids;

//--------------------------------------------------------------

/* GET */
#[warn(dead_code)]
pub async fn get_many_notes(
    query: web::Query<QuerysParams>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let (search, fields_search, search_query) = get_search::<Note>(&query);
    let (fields, without, valid_fields) = get_fields::<Note>(&query);
    let (sort, sort_field, sort_order) = get_sort::<Note>(&query);
    let (limit, pag, offset) = get_pagination::<Note>(&query);

    let mut stmt = include_str!("./querys/get_notes.sql").to_string();

    stmt = stmt
        .replace("$offset_pag", &offset.to_string())
        .replace("$limit_pag", &limit.to_string())
        .replace("$table_fields", &valid_fields)
        .replace("$_SEARCH_", &search_query)
        .replace("$sort_field", &sort_field)
        .replace("$sort_order", &sort_order);

    let stmt = client.prepare(&stmt).await.unwrap();

    let results = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Note::from_row_option(row, &valid_fields).unwrap())
        .map(|note| note.to_filtered_map())
        .collect::<VecJson>();

    let count_total = Note::get_count_total(&client).await.unwrap_or(0);
    let count = results.len();

    let response_data = get_response(GetResponseParams {
        fields_search,
        count_total,
        results,
        without,
        search,
        fields,
        count,
        limit,
        pag,
        sort,
    });

    return Ok(HttpResponse::Ok().json(response_data));
}

/* GET */
#[warn(dead_code)]
pub async fn get_one_note(
    id: web::Path<i32>,
    query: web::Query<QuerysParams>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let mut stmt = include_str!("./querys/get_one_note.sql").to_string();

    let (_, _, valid_fields) = get_fields::<Note>(&query);

    stmt = stmt
        .replace("$table_fields", &valid_fields)
        .replace("$id_note", &id.to_string());
    let stmt = client.prepare(&stmt).await.unwrap();

    let results = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Note::from_row_option(row, &valid_fields).unwrap())
        .map(|note| note.to_filtered_map())
        .collect::<VecJson>();

    if results.is_empty() {
        return Err(MyError::NotFound);
    }

    return Ok(HttpResponse::Ok().json(results.get(0)));
}

/* POST */
#[warn(dead_code)]
pub async fn create_one_note(
    body: web::Json<NoteUseCreate>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let body_params: NoteUseCreate = body.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let stmt = include_str!("./querys/add_note.sql");
    let stmt = client.prepare(&stmt).await.unwrap();

    let result = client
        .query(
            &stmt,
            &[
                &body_params.title,
                &body_params.details,
                &body_params.done.unwrap_or(false),
            ],
        )
        .await?
        .iter()
        .map(|row| Note::from_row_ref(row).unwrap())
        .collect::<Vec<Note>>()
        .pop()
        .ok_or(MyError::NotFound);

    return Ok(HttpResponse::Ok().json(result.unwrap()));
}

/* PUT */
#[warn(dead_code)]
pub async fn update_one_note(
    body: web::Json<NoteUseUpdate>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let body_params: NoteUseUpdate = body.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let mut stmt = include_str!("./querys/update_note.sql").to_string();
    let mut placeholders = Vec::new();
    let mut values: Vec<&(dyn ToSql + Sync)> = Vec::new();

    values.push(&body_params.id);

    if body_params.title.is_some() {
        placeholders.push(format!("title = ${}", values.len() + 1).to_string());
        values.push(&body_params.title);
    }

    if body_params.details.is_some() {
        placeholders.push(format!("details = ${}", values.len() + 1).to_string());
        values.push(&body_params.details);
    }

    if body_params.done.is_some() {
        placeholders.push(format!("done = ${}", values.len() + 1).to_string());
        values.push(&body_params.done);
    }

    let set_clause = placeholders.join(", ");
    stmt = stmt.replace("$set_clause", &set_clause);
    let stmt = client.prepare(&stmt).await.unwrap();

    let rows = client.query(&stmt, &values).await?;

    if let Some(row) = rows.first() {
        let note = Note::from_row(row.clone())?;
        Ok(HttpResponse::Ok().json(note))
    } else {
        Err(MyError::NotFound)
    }
}

/* DELETE */
#[warn(dead_code)]
pub async fn delete_many_notes(
    body: web::Json<Ids>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let ids_params: Vec<i64> = body.ids.iter().map(|id| *id as i64).collect();
    let placeholders: Vec<String> = (1..=ids_params.len()).map(|i| format!("${}", i)).collect();
    let ids = ids_params
        .iter()
        .map(|id| id as &(dyn ToSql + Sync))
        .collect::<Vec<_>>();

    let mut stmt = include_str!("./querys/delete_notes.sql").to_string();
    stmt = stmt.replace("$ids", &placeholders.join(", "));
   
    let rows = client.query(&stmt, &ids).await?;

   if let Some(row) = rows.first() {
        let note = Note::from_row(row.clone())?;
        Ok(HttpResponse::Ok().json(note))
    } else {
        Err(MyError::NotFound)
    }
}
