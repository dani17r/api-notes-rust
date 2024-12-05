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

use crate::{module::default::models::FieldOperations, utils::querys::{DbFields, Fields}};

#[derive(Debug, Deserialize, Serialize)]
pub struct TagVec(pub Vec<Tag>);

#[derive(Default, Debug, Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "tags")]
pub struct Tag {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TagUseCreate {
    pub name: String,
    pub description: String,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TagUseUpdate {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
}

#[derive(Deserialize)]
pub struct Ids {
    pub ids: Vec<i64>,
}

impl FromSql<'_> for TagVec {
    fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let tags: Vec<Tag> = FieldOperations::from_sql_json(ty, raw)?;
        Ok(TagVec(tags))
    }

    fn accepts(ty: &Type) -> bool {
        *ty == Type::JSONB || *ty == Type::JSON
    }
}

impl FromSql<'_> for Tag {
    fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let tag: Tag = FieldOperations::from_sql_json(ty, raw)?;
        Ok(tag)
    }

    fn accepts(ty: &Type) -> bool {
        *ty == Type::JSONB
    }
}

impl Tag {
    pub fn from_row_option(row: &Row, fields: &str) -> Result<Tag, &'static str> {
        let _fields_: Vec<&str> = fields.split(',').map(|f| f.trim()).collect();

        println!("{:?} -- {}", _fields_, fields);
        let mut note = Tag::default();
        note.id = row.get("id");

        for field in _fields_ {
            match field {
                "tags.name" => {
                    if fields.contains(&"tags.name") {
                        note.name = row.get("name");
                    } else {
                        note.name = None;
                    }
                }
                "tags.description" => {
                    if fields.contains(&"tags.description") {
                        note.description = row.get("description")
                    } else {
                        note.description = None;
                    }
                }
                "tags.color" => {
                    if fields.contains(&"tags.color") {
                        note.color = row.get("color")
                    } else {
                        note.color = None;
                    }
                }
                _ => continue,
            }
        }
        Ok(note)
    }

    pub fn to_filtered_map(&self) -> OrderMap<String, Value> {
        let mut map = OrderMap::new();

        map.insert("id".to_string(), serde_json::json!(self.id));

        if let Some(name) = &self.name {
            map.insert("name".to_string(), serde_json::json!(name));
        }

        if let Some(description) = &self.description {
            map.insert("description".to_string(), serde_json::json!(description));
        }

        if let Some(color) = &self.color {
            map.insert("color".to_string(), serde_json::json!(color));
        }

        return map.into();
    }

    pub async fn get_count_total(client: &Client) -> Result<i64, Box<dyn std::error::Error>> {
        let stmt_count = include_str!("./querys/count_tags.sql").to_string();
        let stmt_count = client.prepare(&stmt_count).await?;

        let row = client.query_one(&stmt_count, &[]).await?;
        let count: i64 = row.get(0);
        Ok(count)
    }

    pub fn get_fields_string() -> Fields {
        Fields {
            db: DbFields {
                all: "tags.id, tags.name, tags.description, tags.color".to_string(),
                without_id: "tags.name, tags.description, tags.color".to_string(),
                // without_ship: "".to_string(),
                without_id_ship: "".to_string(),
            },
            // normal: (
            //    "id, name, description, color".to_string(),
            //    "name, description, color".to_string(),
            //    "".to_string()
            // ),
            searchs: "name, description, color".to_string(),
            conditionals: "".to_string(),
        }
    }
}
