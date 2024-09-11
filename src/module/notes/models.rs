use deadpool_postgres::Client;
use ordermap::OrderMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::Row;

#[derive(Default, Debug, Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "notes")]
pub struct Note {
    pub id: i64,
    pub title: Option<String>,
    pub details: Option<String>,
    pub done: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NoteUseCreate {
    pub title: String,
    pub details: String,
    pub done: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NoteUseUpdate {
    pub id: i64,
    pub title: Option<String>,
    pub details: Option<String>,
    pub done: Option<bool>,
}

impl Note {
    pub fn from_row_option(row: &Row, fields: &str) -> Result<Note, &'static str> {
        let _fields_: Vec<&str> = fields.split(',').collect();

        let mut note = Note::default();
        note.id = row.get("id");

        if !fields.contains("*") {
            for field in _fields_ {
                match field {
                    "title" => {
                        if fields.contains(&"title") {
                            note.title = row.get("title");
                        } else {
                            note.title = None;
                        }
                    }
                    "details" => {
                        if fields.contains(&"details") {
                            note.details = row.get("details")
                        } else {
                            note.details = None;
                        }
                    }
                    "done" => {
                        if fields.contains(&"done") {
                            note.done = row.get("done")
                        } else {
                            note.done = None;
                        }
                    }
                    _ => continue,
                }
            }
        } else {
            note.title = row.get("title");
            note.details = row.get("details");
            note.done = row.get("done");
        }

        Ok(note)
    }

    pub fn to_filtered_map(&self) -> OrderMap<String, Value> {
        let mut map = OrderMap::new();

        map.insert("id".to_string(), serde_json::json!(self.id));

        if let Some(title) = &self.title {
            map.insert("title".to_string(), serde_json::json!(title));
        }

        if let Some(details) = &self.details {
            map.insert("details".to_string(), serde_json::json!(details));
        }

        if let Some(done) = &self.done {
            map.insert("done".to_string(), serde_json::json!(done));
        }

        return map.into();
    }

    pub async fn get_count_total(client: &Client) -> Result<i64, Box<dyn std::error::Error>> {
        let stmt_count = include_str!("./querys/count_notes.sql").to_string();
        let stmt_count = client.prepare(&stmt_count).await?;

        let row = client.query_one(&stmt_count, &[]).await?;
        let count: i64 = row.get(0);
        Ok(count)
    }
}
