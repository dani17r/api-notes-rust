use super::types::VecJson;
use serde::Serialize;

//--------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct PaginationResponse {
    pub count_total: i64,
    pub count: usize,
    pub pag: u8,
    pub limit: u8,
}

#[derive(Debug, Serialize)]
pub struct FilterResponse {
    pub fields_search: String,
    pub search: String,
    pub fields: String,
    pub without: bool,
    pub sort: String,
}

#[derive(Debug, Serialize)]
pub struct ResponseData {
    pub data: VecJson,
    pub pagination: PaginationResponse,
    pub filters: FilterResponse,
}

pub struct GetResponseParams {
    pub count_total: i64,
    pub count: usize,
    pub pag: u8,
    pub limit: u8,
    pub fields_search: String,
    pub search: String,
    pub fields: String,
    pub without: bool,
    pub sort: String,
    pub results: VecJson,
}

#[derive(Debug)]
pub struct FieldOperations;

impl FieldOperations {
    pub fn get_fields(fields_table: &String) -> Vec<&str> {
        let value_fields = fields_table
            .split(',')
            .filter_map(|s| {
                let trimmed = s.trim();
                let field_parts: Vec<&str> = trimmed.split('.').collect();
                if let Some(field_name) = field_parts.last() {
                    Some(field_name.trim())
                } else {
                    None
                }
            })
            .filter(|x| !x.contains(&"id"))
            .collect();

        return value_fields;
    }

    pub fn get_select_fields<'a>(valid_fields: &Vec<&'a str>, fields: &'a String) -> Vec<&'a str> {
        let selected_fields = fields
            .split(',')
            .filter_map(|f| {
                let field = f.trim().split(".").last().unwrap().trim();
                if valid_fields.contains(&field) {
                    Some(field)
                } else {
                    None
                }
            })
            .collect();

        return selected_fields;
    }
}
