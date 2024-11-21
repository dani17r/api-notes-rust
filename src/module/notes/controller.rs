use crate::{
    core::database::errors::MyError,
    module::{
        default::{
            models::{FieldOperations, GetResponseParams},
            types::{QuerysParams, VecJson},
        },
        notes::models::{Ids, Note, NoteUseCreate, NoteUseUpdate},
    },
    utils::querys::{get_fields, get_pagination, get_response, get_search, get_sort},
};
use actix_web::{web, HttpResponse};
use deadpool_postgres::{Client, GenericClient, Pool};
use tokio_postgres::types::ToSql;

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
    let mut validate_relationship = "".to_string();
    let consult_valid_fields: String;

    let valid_fields_with_ship: Vec<String> =
        FieldOperations::get_fields_iterator(&valid_fields, &"notes.".to_string()).collect();

    let valid_fields_without_ship: Vec<String> =
        FieldOperations::get_fields_iterator(&valid_fields, &"notes.".to_string())
            .filter(|x| !x.contains("tags"))
            .collect();

    if valid_fields.contains(&"tags".to_string()) {
        let mut stmt_tags =
            include_str!("../notes_tags/querys/get_tags_relationship.sql").to_string();
        stmt_tags = stmt_tags.replace(
            "$fields_relationship",
            &", 'name', tags.name, 'description', tags.description, 'color', tags.color",
        );
        validate_relationship = format!(", {}", &stmt_tags);
    }

    if valid_fields_without_ship.len() >= 1 {
        consult_valid_fields = format!(", {}", valid_fields_without_ship.join(", ")).to_string();
    } else {
        consult_valid_fields = valid_fields_without_ship.join(", ").to_string();
    }

    stmt = stmt
        .replace("$table_fields", &consult_valid_fields)
        .replace("$relationship", &validate_relationship)
        .replace("$offset_pag", &offset.to_string())
        .replace("$limit_pag", &limit.to_string())
        .replace("$_SEARCH_", &search_query)
        .replace("$sort_field", &sort_field)
        .replace("$sort_order", &sort_order);

    let stmt = client.prepare(&stmt).await.unwrap();

    let results = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Note::from_row_option(row, &valid_fields_with_ship.join(", ")).unwrap())
        .map(|note| note.to_filtered_map())
        .collect::<VecJson>();

    let count_total = Note::get_count_total(&client).await.unwrap_or(0);
    let count = results.len();

    let response_data = get_response(GetResponseParams {
        results: results.clone(),
        fields_search,
        count_total,
        without,
        search,
        fields,
        count,
        limit,
        pag,
        sort,
    });

    if results.is_empty() {
        return Err(MyError::NotFound);
    }

    Ok(HttpResponse::Ok().json(response_data))
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
    let mut validate_relationship = "".to_string();
    let consult_valid_fields: String;

    let (_, _, valid_fields) = get_fields::<Note>(&query);
    let valid_fields_without_ship: Vec<String> =
        FieldOperations::get_fields_iterator(&valid_fields, &"notes.".to_string())
            .filter(|x| !x.contains("tags"))
            .collect();

    if valid_fields.contains(&"tags".to_string()) {
        let mut stmt_tags =
            include_str!("../notes_tags/querys/get_tags_relationship.sql").to_string();
        stmt_tags = stmt_tags.replace(
            "$fields_relationship",
            &", 'name', tags.name, 'description', tags.description, 'color', tags.color",
        );
        validate_relationship = format!(", {}", &stmt_tags);
    }

    if valid_fields_without_ship.len() >= 1 {
        consult_valid_fields = format!(", {}", valid_fields_without_ship.join(", ")).to_string();
    } else {
        consult_valid_fields = valid_fields_without_ship.join(", ").to_string();
    }

    stmt = stmt
        .replace("$table_fields", &consult_valid_fields)
        .replace("$relationship", &validate_relationship)
        .replace("$id_note", &id.to_string());
    let stmt = client.prepare(&stmt).await.unwrap();

    let results = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| Note::from_row_option(row, &valid_fields).unwrap())
        .collect::<Vec<Note>>();

    if results.is_empty() {
        return Err(MyError::NotFound);
    }

    return Ok(HttpResponse::Ok().json(results.first()));
}

/* POST */
#[warn(dead_code)]
pub async fn create_one_note(
    body: web::Json<NoteUseCreate>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let body_params: NoteUseCreate = body.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let valid_fields_with_ship: Vec<String> =
        FieldOperations::get_fields_iterator(&"title,details,done,rank", &"notes.".to_string())
            .collect();

    let stmt = include_str!("./querys/add_note.sql");
    let stmt = client.prepare(&stmt).await.unwrap();

    let result = client
        .query(
            &stmt,
            &[
                &body_params.title,
                &body_params.details,
                &body_params.done.unwrap_or(false),
                &body_params.rank.unwrap_or(0),
            ],
        )
        .await?
        .iter()
        .map(|row| Note::from_row_option(row, &valid_fields_with_ship.join(", ")).unwrap())
        .collect::<Vec<Note>>();

    if result.is_empty() {
        return Err(MyError::NotFound);
    }

    Ok(HttpResponse::Ok().json(result.first()))
}

/* PUT */
#[warn(dead_code)]
pub async fn update_one_note(
    body: web::Json<NoteUseUpdate>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let body_params: NoteUseUpdate = body.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
    let valid_fields_with_ship: Vec<String> =
        FieldOperations::get_fields_iterator(&"title,details,done,rank", &"notes.".to_string())
            .collect();

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

    if body_params.rank.is_some() {
        placeholders.push(format!("rank = ${}", values.len() + 1).to_string());
        values.push(&body_params.rank);
    }

    let set_clause = placeholders.join(", ");
    stmt = stmt.replace("$set_clause", &set_clause);
    let stmt = client.prepare(&stmt).await.unwrap();

    let result = client
        .query(&stmt, &values)
        .await?
        .iter()
        .map(|row| Note::from_row_option(row, &valid_fields_with_ship.join(", ")).unwrap())
        .collect::<Vec<Note>>();

    if result.is_empty() {
        return Err(MyError::NotFound);
    }

    Ok(HttpResponse::Ok().json(result.first()))
}

/* DELETE */
#[warn(dead_code)]
pub async fn delete_many_notes(
    body: web::Json<Ids>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, MyError> {
    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let valid_fields_with_ship: Vec<String> =
        FieldOperations::get_fields_iterator(&"title, details, done, rank", &"notes.".to_string())
            .collect();
    let ids_params: Vec<i64> = body.ids.iter().map(|id| *id as i64).collect();
    let placeholders: Vec<String> = (1..=ids_params.len()).map(|i| format!("${}", i)).collect();
    let ids = ids_params
        .iter()
        .map(|id| id as &(dyn ToSql + Sync))
        .collect::<Vec<_>>();

    let mut stmt = include_str!("./querys/delete_notes.sql").to_string();
    stmt = stmt.replace("$ids", &placeholders.join(", "));

    let results = client
        .query(&stmt, &ids)
        .await?
        .iter()
        .map(|row| Note::from_row_option(row, &valid_fields_with_ship.join(", ")).unwrap())
        .map(|note| note.to_filtered_map())
        .collect::<VecJson>();

    if results.is_empty() {
        return Err(MyError::NotFound);
    }

    Ok(HttpResponse::Ok().json(results))
}
