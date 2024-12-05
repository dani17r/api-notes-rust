use ordermap::OrderMap;
use serde::Deserialize;
use serde_json::Value;

//--------------------------------------------------------------

#[derive(Debug, Deserialize, Default)]
pub struct QuerysParams {
    pub fields_search: Option<String>,
    pub conditionals: Option<String>,
    pub search: Option<String>,
    pub fields: Option<String>,
    pub without: Option<bool>,
    pub sort: Option<String>,
    pub limit: Option<u8>,
    pub pag: Option<u8>,
}

pub type VecJson = Vec<OrderMap<String, Value>>;
