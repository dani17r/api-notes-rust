use crate::{
    core::database::errors::MyError,
    module::{
        default::{
            models::GetResponseParams,
            types::{QuerysParams, VecJson},
        },
        tags::models::{Tag, TagUseCreate, TagUseUpdate},
    },
    utils::querys::{get_params, get_pagination, get_response, get_search, get_sort},
};
use actix_web::{web, HttpResponse};
use deadpool_postgres::{Client, GenericClient, Pool};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_postgres::types::ToSql;

use super::models::Ids;

//--------------------------------------------------------------

/* GET */
#[warn(dead_code)]
pub async fn get_many_tags(
    query: web::Query<QuerysParams>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let fields_string= Tag::get_fields_string();

    let (search, fields_search, search_query) = get_search(&fields_string,&query);
    let (fields, without, valid_fields) = get_params(&fields_string, &query);
    let (sort, sort_field, sort_order) = get_sort(&query);
    let (limit, pag, offset) = get_pagination(&query);

    let mut stmt = include_str!("./querys/get_tags.sql").to_string();

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
        .map(|row| Tag::from_row_option(row, &valid_fields).unwrap())
        .map(|tag| tag.to_filtered_map())
        .collect::<VecJson>();

    let count_total = Tag::get_count_total(&client).await.unwrap_or(0);
    let count = results.len();

    let response_data = get_response(GetResponseParams {
        fields_search,
        count_total,
        conditionals: "".to_string(),
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
pub async fn get_one_tag(
    id: web::Path<i32>,
    query: web::Query<QuerysParams>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let fields_string= Tag::get_fields_string();

    let mut stmt = include_str!("./querys/get_one_tag.sql").to_string();

    let (_, _, valid_fields) = get_params(&fields_string, &query);

    stmt = stmt
        .replace("$table_fields", &valid_fields)
        .replace("$id_tag", &id.to_string());
    let stmt = client.prepare(&stmt).await.unwrap();

    let results = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Tag::from_row_option(row, &valid_fields).unwrap())
        .collect::<Vec<Tag>>();

    if results.is_empty() {
        return Err(MyError::NotFound);
    }

    return Ok(HttpResponse::Ok().json(results.get(0)));
}

/* POST */
#[warn(dead_code)]
pub async fn create_one_tag(
    body: web::Json<TagUseCreate>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let body_params: TagUseCreate = body.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let stmt = include_str!("./querys/add_tag.sql");
    let stmt = client.prepare(&stmt).await.unwrap();

    let result = client
        .query(
            &stmt,
            &[
                &body_params.name,
                &body_params.description,
                &body_params.color,
            ],
        )
        .await?
        .iter()
        .map(|row| Tag::from_row_ref(row).unwrap())
        .collect::<Vec<Tag>>()
        .pop()
        .ok_or(MyError::NotFound);

    return Ok(HttpResponse::Ok().json(result.unwrap()));
}

/* PUT */
#[warn(dead_code)]
pub async fn update_one_tag(
    body: web::Json<TagUseUpdate>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let body_params: TagUseUpdate = body.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let mut stmt = include_str!("./querys/update_tag.sql").to_string();
    let mut placeholders = Vec::new();
    let mut values: Vec<&(dyn ToSql + Sync)> = Vec::new();

    values.push(&body_params.id);

    if body_params.name.is_some() {
        placeholders.push(format!("name = ${}", values.len() + 1).to_string());
        values.push(&body_params.name);
    }

    if body_params.description.is_some() {
        placeholders.push(format!("description = ${}", values.len() + 1).to_string());
        values.push(&body_params.description);
    }

    if body_params.color.is_some() {
        placeholders.push(format!("color = ${}", values.len() + 1).to_string());
        values.push(&body_params.color);
    }

    let set_clause = placeholders.join(", ");
    stmt = stmt.replace("$set_clause", &set_clause);
    let stmt = client.prepare(&stmt).await.unwrap();

    let rows = client.query(&stmt, &values).await?;

    if let Some(row) = rows.first() {
        let tag = Tag::from_row(row.clone())?;
        Ok(HttpResponse::Ok().json(tag))
    } else {
        Err(MyError::NotFound)
    }
}

/* DELETE */
#[warn(dead_code)]
pub async fn delete_many_tags(
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

    let mut stmt = include_str!("./querys/delete_tags.sql").to_string();
    stmt = stmt.replace("$ids", &placeholders.join(", "));

    let mut stmt_relation = include_str!("../notes_tags/querys/delete_tags_in_note_tags.sql").to_string();
    stmt_relation = stmt_relation.replace("$ids", &placeholders.join(", "));
   
    let rows = client.query(&stmt, &ids).await?;
    client.query(&stmt_relation, &ids).await?;

   if let Some(row) = rows.first() {
        let tag = Tag::from_row(row.clone())?;
        Ok(HttpResponse::Ok().json(tag))
    } else {
        Err(MyError::NotFound)
    }
}
