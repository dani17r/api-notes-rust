use deadpool_postgres::Client;
use ordermap::OrderMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::{
    types::{FromSql, Type},
    Row,
};

use crate::module::default::models::FieldOperations;

#[derive(Debug, Deserialize, Serialize)]
pub struct CategoryVec(pub Vec<Category>);

#[derive(Default, Debug, Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "categories")]
pub struct Category {
    pub id: i64,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CategoryUseCreate {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CategoryUseUpdate {
    pub id: i64,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct Ids {
    pub ids: Vec<i64>,
}

impl FromSql<'_> for CategoryVec {
    fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let categories: Vec<Category> = FieldOperations::from_sql_json(ty, raw)?;
        Ok(CategoryVec(categories))
    }

    fn accepts(ty: &Type) -> bool {
        *ty == Type::JSONB || *ty == Type::JSON
    }
}

impl FromSql<'_> for Category {
    fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let category: Category = FieldOperations::from_sql_json(ty, raw)?;
        Ok(category)
    }

    fn accepts(ty: &Type) -> bool {
        *ty == Type::JSONB
    }
}

impl Category {
    pub fn from_row_option(row: &Row, fields: &str) -> Result<Category, &'static str> {
        let _fields_: Vec<&str> = fields.split(',').map(|f| f.trim()).collect();

        println!("{:?} -- {}", _fields_, fields);
        let mut category = Category::default();
        category.id = row.get("id");

        for field in _fields_ {
            match field {
                "categories.title" => {
                    if fields.contains(&"categories.title") {
                        category.title = row.get("title");
                    } else {
                        category.title = None;
                    }
                }
                "categories.description" => {
                    if fields.contains(&"categories.description") {
                        category.description = row.get("description")
                    } else {
                        category.description = None;
                    }
                }
                _ => continue,
            }
        }
        Ok(category)
    }

    pub fn to_filtered_map(&self) -> OrderMap<String, Value> {
        let mut map = OrderMap::new();

        map.insert("id".to_string(), serde_json::json!(self.id));

        if let Some(title) = &self.title {
            map.insert("title".to_string(), serde_json::json!(title));
        }

        if let Some(description) = &self.description {
            map.insert("description".to_string(), serde_json::json!(description));
        }

        return map.into();
    }

    pub async fn get_count_total(client: &Client) -> Result<i64, Box<dyn std::error::Error>> {
        let stmt_count = include_str!("./querys/count_categories.sql").to_string();
        let stmt_count = client.prepare(&stmt_count).await?;

        let row = client.query_one(&stmt_count, &[]).await?;
        let count: i64 = row.get(0);
        Ok(count)
    }
}
