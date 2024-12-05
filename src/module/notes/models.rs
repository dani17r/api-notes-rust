use deadpool_postgres::Client;
use ordermap::OrderMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_pg_mapper_derive::PostgresMapper;
use tokio_postgres::Row;

use crate::{module::tags::models::{Tag, TagVec}, utils::querys::{DbFields, Fields}};

#[derive(Default, Debug, Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "notes")]
pub struct Note {
    pub id: i64,
    pub title: Option<String>,
    pub details: Option<String>,
    pub done: Option<bool>,
    pub rank: Option<i64>,
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NoteUseCreate {
    pub title: String,
    pub details: String,
    pub done: Option<bool>,
    pub rank: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AddTagsInNote {
    pub tag_ids: Option<Vec<i64>>,
    pub note_id: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NoteUseUpdate {
    pub id: i64,
    pub title: Option<String>,
    pub details: Option<String>,
    pub done: Option<bool>,
    pub rank: Option<i64>,
}

#[derive(Deserialize)]
pub struct Ids {
    pub ids: Vec<i64>,
}

impl Note {
    pub fn from_row_option(row: &Row, fields: &str) -> Result<Note, &'static str> {
        let _fields_: Vec<&str> = fields.split(',').map(|f| f.trim()).collect();

        let mut note = Note::default();
        note.id = row.get("id");

        for field in _fields_ {
            match field {
                "notes.title" => {
                    if fields.contains(&"notes.title") {
                        note.title = row.get("title");
                    } else {
                        note.title = None;
                    }
                }
                "notes.details" => {
                    if fields.contains(&"notes.details") {
                        note.details = row.get("details");
                    } else {
                        note.details = None;
                    }
                }
                "notes.done" => {
                    if fields.contains(&"notes.done") {
                        note.done = row.get("done");
                    } else {
                        note.done = None;
                    }
                }
                "notes.rank" => {
                    if fields.contains(&"notes.rank") {
                        note.rank = row.get("rank");
                    } else {
                        note.rank = None;
                    }
                }
                "notes.tags" => {
                    if fields.contains(&"notes.tags") {
                        let tags: Option<TagVec> = row.get("tags");
                        note.tags = tags.map(|tag_vec| tag_vec.0);
                        // note.tags = None;
                    } else {
                        note.tags = None;
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

        if let Some(title) = &self.title {
            map.insert("title".to_string(), serde_json::json!(title));
        }

        if let Some(details) = &self.details {
            map.insert("details".to_string(), serde_json::json!(details));
        }

        if let Some(done) = &self.done {
            map.insert("done".to_string(), serde_json::json!(done));
        }

        if let Some(rank) = &self.rank {
            map.insert("rank".to_string(), serde_json::json!(rank));
        }

        if let Some(tags) = &self.tags {
            map.insert("tags".to_string(), serde_json::json!(tags));
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

    pub fn get_fields_string() -> Fields {
        Fields {
            db: DbFields {
                all: "notes.id, notes.title, notes.details, notes.done, notes.rank, notes.tags".to_string(),
                without_id: "notes.title, notes.details, notes.done, notes.rank, notes.tags".to_string(),
                // without_ship: "notes.id, notes.title, notes.details, notes.done, notes.rank".to_string(),
                without_id_ship: "notes.title, notes.details, notes.done, notes.rank".to_string(),
            },
            // normal: {
            //     "id, title, details, done, rank, tags".to_string(),
            //     "title, details, done, rank, tags".to_string(),
            //     "title, details, done, rank".to_string(),
            // },
            searchs: "title, details".to_string(),
            conditionals: "done, rank".to_string(),
        }
    }
}
