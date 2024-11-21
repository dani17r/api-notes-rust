use crate::module::default::{
    models::{
        FieldOperations, FilterResponse, GetResponseParams, PaginationResponse, ResponseData,
    },
    types::QuerysParams,
};
use actix_web::web;
// use std::collections::HashSet;
use tokio_pg_mapper::FromTokioPostgresRow;
//--------------------------------------------------------------

// fn remove_duplicates(vec: Vec<String>) -> Vec<String> {
//     let set: HashSet<String> = vec.into_iter().collect(); // Convierte el vector en un HashSet
//     return set.into_iter().collect(); // Convierte el HashSet de nuevo a un vector
// }

pub fn get_search<T: FromTokioPostgresRow>(
    query: &web::Query<QuerysParams>,
) -> (String, String, String) {
    let fields_search = query.fields_search.as_deref().unwrap_or("");
    let search = query.search.as_deref().unwrap_or("");

    if !search.is_empty() {
        let fields_table = T::sql_table_fields();
        let mut valid_fields = FieldOperations::get_fields(&fields_table);

        if !fields_search.is_empty() {
            let new_valid_fields = fields_search.split(":").collect();
            let valid_fields_new =
                FieldOperations::get_select_fields(&new_valid_fields, &fields_table);

            if !valid_fields_new.is_empty() {
                valid_fields = valid_fields_new;
            }
        }

        let query_search = format!(
            "WHERE CONCAT({}) ILIKE '%{}%'",
            &valid_fields.join(","),
            &search
        );

        return (search.to_string(), fields_search.to_string(), query_search);
    } else {
        return (
            search.to_string(),
            fields_search.to_string(),
            "".to_string(),
        );
    }
}

pub fn get_pagination<T: FromTokioPostgresRow>(query: &web::Query<QuerysParams>) -> (u8, u8, u8) {
    let limit = query.limit.unwrap_or(10);
    let pag = query.pag.unwrap_or(1);
    let offset = (pag - 1) * limit;

    return (limit, pag, offset);
}

pub fn get_sort<T: FromTokioPostgresRow>(
    query: &web::Query<QuerysParams>,
) -> (String, String, String) {
    const ORDER_DEFAULT: &str = "ASC";
    const DEFAULT_SORT: &str = "id:ASC";

    let sort = &query.sort.as_deref().unwrap_or(DEFAULT_SORT);

    if !sort.is_empty() {
        if sort.contains(":") {
            let sort_parts: Vec<&str> = sort.split(':').collect();
            let order = match sort_parts[1].to_lowercase().as_str() {
                "asc" => "ASC",
                "desc" => "DESC",
                _ => ORDER_DEFAULT,
            };

            return (
                sort.to_string(),
                sort_parts[0].to_string(),
                order.to_uppercase().to_string(),
            );
        } else {
            return (
                sort.to_string(),
                sort.to_string(),
                ORDER_DEFAULT.to_string(),
            );
        }
    } else {
        let sort_parts: Vec<&str> = DEFAULT_SORT.split(':').collect();
        return (
            sort.to_string(),
            sort_parts[0].to_string(),
            sort_parts[1].to_uppercase().to_string(),
        );
    }
}

pub fn get_fields<T: FromTokioPostgresRow>(
    query: &web::Query<QuerysParams>,
) -> (String, bool, String) {
    let fields = &query.fields.as_deref().unwrap_or("");
    let without = query.without.unwrap_or(false);

    let fields_select: Vec<String>;

    if !fields.is_empty() {
        let fields_table = T::sql_table_fields();
        let valid_fields = FieldOperations::get_fields(&fields_table);
        let field_with_prefix: Vec<&str> = fields.split(",").collect();
        let selected_fields = FieldOperations::get_select_fields(&field_with_prefix, &fields_table);

        if selected_fields.is_empty() {
            fields_select = valid_fields
                .iter()
                .map(|field| field.trim().to_string())
                .collect();

            return (
                fields.to_string(),
                without,
                fields_select.join(", ").to_string(),
            );
        } else {
            if without {
                let filtered_fields: Vec<String> = fields_table
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|x| !x.contains(&"id"))
                    .filter(|field| {
                        let new_field = field.trim().split(".").last().unwrap();
                        !selected_fields.join(", ").contains(new_field.trim())
                    })
                    .collect();

                return (
                    fields.to_string(),
                    without,
                    filtered_fields.join(", ").trim().to_string(),
                );
            } else {
                return (
                    fields.to_string(),
                    without,
                    selected_fields.join(", ").trim().to_string(),
                );
            }
        }
    } else {
        let fields_table = T::sql_table_fields();
        let filtered_fields: Vec<_> = fields_table
            .split(',')
            .map(|s| s.trim())
            .filter(|x| !x.contains(&"id"))
            .collect();

        return (fields.to_string(), without, filtered_fields.join(", "));
    }
}

pub fn get_response<T>(data: GetResponseParams<T>) -> ResponseData<T> {
    let pagination = PaginationResponse {
        count_total: data.count_total,
        count: data.count,
        limit: data.limit,
        pag: data.pag,
    };

    let filters = FilterResponse {
        fields_search: data.fields_search,
        without: data.without,
        fields: data.fields,
        search: data.search,
        sort: data.sort,
    };

    return ResponseData {
        data: data.results,
        pagination,
        filters,
    };
}
