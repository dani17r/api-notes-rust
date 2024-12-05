use super::types::VecJson;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use tokio_postgres::types::Type;

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
    pub conditionals: String,
    pub search: String,
    pub fields: String,
    pub without: bool,
    pub sort: String,
}

#[derive(Debug, Serialize)]
pub struct ResponseData<T = VecJson> {
    pub data: T,
    pub pagination: PaginationResponse,
    pub filters: FilterResponse,
}

pub struct GetResponseParams<T = VecJson> {
    pub count_total: i64,
    pub count: usize,
    pub pag: u8,
    pub limit: u8,
    pub fields_search: String,
    pub conditionals: String,
    pub search: String,
    pub fields: String,
    pub without: bool,
    pub sort: String,
    pub results: T,
}

#[derive(Debug)]
pub struct FieldOperations;

impl FieldOperations {
    pub fn from_sql_json<T: for<'de> Deserialize<'de>>(
        ty: &Type,
        raw: &[u8],
    ) -> Result<T, Box<dyn Error + Send + Sync>> {
        if *ty != Type::JSONB && *ty != Type::JSON {
            return Err("Incompatible type".into());
        }

        let json: Value = serde_json::from_slice(raw)?;
        if !json.is_array() {
            return Err("Expected a JSON array".into());
        }

        serde_json::from_value(json).map_err(Into::into)
    }

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

    pub fn get_select_fields<'a>(fields: &Vec<&'a str>, fields_table: &'a String) -> Vec<&'a str> {
        let selected_fields = fields_table
            .split(',')
            .filter_map(|f| {
                let field = f.trim().split(".").last().unwrap().trim();
                if fields.contains(&field) {
                    Some(field)
                } else {
                    None
                }
            })
            .collect();

        return selected_fields;
    }

     pub fn get_fields_iterator<'a>(fields: &'a str, verify_str: &'a str) -> impl Iterator<Item = String> + 'a {
        fields.split(',').map(move |s| {
            let trimmed = s.trim();
            if trimmed.contains(verify_str) {
                trimmed.to_string()
            } else {
                format!("{}{}", verify_str, trimmed)
            }
        })
    }
}
