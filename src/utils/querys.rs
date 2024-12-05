use crate::module::default::{
    models::{
        FieldOperations, FilterResponse, GetResponseParams, PaginationResponse, ResponseData,
    },
    types::QuerysParams,
};
use actix_web::web;
// use std::collections::HashSet;
// use tokio_pg_mapper::FromTokioPostgresRow;
//--------------------------------------------------------------

// fn remove_duplicates(vec: Vec<String>) -> Vec<String> {
//     let set: HashSet<String> = vec.into_iter().collect(); // Convierte el vector en un HashSet
//     return set.into_iter().collect(); // Convierte el HashSet de nuevo a un vector
// }

#[allow(dead_code)]
pub struct DbFields {
    pub all: String,
    pub without_id: String,
    // pub without_ship: String,
    pub without_id_ship: String,
}

pub struct Fields {
    pub db: DbFields,
    // pub normal: (String, String, String),
    pub searchs: String,
    pub conditionals: String,
}

pub fn get_search(
    fields: &Fields,
    query: &web::Query<QuerysParams>,
) -> (String, String, String) {
    let fields_search = query.fields_search.as_deref().unwrap_or("");
    let search = query.search.as_deref().unwrap_or("");

    if !search.is_empty() {
        let fields_table = &fields.searchs;
        let mut valid_fields = fields_table.split(",").map(|x| x.trim()).collect();
        
        if !fields_search.is_empty() {
            let fields_search_arr = fields_search.split(":").collect();
            let valid_fields_new = FieldOperations::get_select_fields(&fields_search_arr, &fields_table);

            if !valid_fields_new.is_empty() {
                valid_fields = valid_fields_new;
            }
        }

        let query_search = format!(
            "CONCAT({}) ILIKE '%{}%'",
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

pub fn get_conditionals(
    fields: &Fields,
    query: &web::Query<QuerysParams>,
) -> (String, String) {
    let conditionals = query.conditionals.as_deref().unwrap_or("").to_string();
    let conditionals_vec: Vec<&str> = conditionals.split(',').map(|f|f.trim()).collect();

    if !conditionals.is_empty() {
        let fields_table = &fields.conditionals;
        let mut query_conditionals = Vec::new(); // Usamos un vector para acumular las condiciones

        for iter in conditionals_vec {
            if iter.contains('=') {
                let parts: Vec<&str> = iter.split('=').collect();
                if parts.len() == 2 {
                    let field = parts[0].trim();
                    let condition = parts[1].trim();

                    if fields_table.contains(field) {
                        if condition.starts_with('[') && condition.ends_with(']') {
                            let values: Vec<&str> = condition[1..condition.len()-1].split('-').collect();
                            if values.len() == 2 {
                                query_conditionals.push(format!(
                                    "{} BETWEEN {} AND {}",
                                    field,
                                    values[0].trim(),
                                    values[1].trim()
                                ));
                            }
                        } 
                        else if ["true", "false"].contains(&condition) {
                            query_conditionals.push(format!("{} is {}", field, condition));
                        }
                    }
                }
            }
        }
        let mut where_clause = query_conditionals.join(" AND ");
        if !where_clause.is_empty() {
            where_clause = format!("WHERE {}", &where_clause);
        }
        return (conditionals.to_string(), where_clause);
    } else {
        return ( conditionals.to_string(), "".to_string());
    }
}

pub fn get_pagination(query: &web::Query<QuerysParams>) -> (u8, u8, u8) {
    let limit = query.limit.unwrap_or(10);
    let pag = query.pag.unwrap_or(1);
    let offset = (pag - 1) * limit;

    return (limit, pag, offset);
}

pub fn get_sort(
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

pub fn get_params(
    fields: &Fields,
    query: &web::Query<QuerysParams>,
) -> (String, bool, String) {
    let params = &query.fields.as_deref().unwrap_or("");
    let without = query.without.unwrap_or(false);

    let params_select: Vec<String>;

    if !params.is_empty() {
        let fields_table = &fields.db.without_id;
        let valid_fields = FieldOperations::get_fields(&fields_table);

        let field_with_prefix: Vec<&str> = params.split(",").collect();
        let selected_fields = FieldOperations::get_select_fields(&field_with_prefix, &fields_table);
        
        if selected_fields.is_empty() {
            params_select = valid_fields
                .iter()
                .map(|field| field.trim().to_string())
                .collect();

            return (
                params.to_string(),
                without,
                params_select.join(", ").to_string(),
            );
        } else {
            if without {
                let filtered_params: Vec<String> = fields_table
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|field| {
                        let new_field = field.trim().split(".").last().unwrap();
                        !selected_fields.join(", ").contains(new_field.trim())
                    })
                    .collect();

                return (
                    params.to_string(),
                    without,
                    filtered_params.join(", ").trim().to_string(),
                );
            } else {
                return (
                    params.to_string(),
                    without,
                    selected_fields.join(", ").trim().to_string(),
                );
            }
        }
    } else {
        let fields_table = &fields.db.without_id;
        let filtered_fields: Vec<_> = fields_table
            .split(',')
            .map(|s| s.trim())
            .collect();

        return (params.to_string(), without, filtered_fields.join(", "));
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
        conditionals: data.conditionals,
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
