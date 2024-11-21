use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;


#[derive(Default, Debug, Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "note_tags")]
pub struct NoteTags {
    pub id: i64,
    pub note_id: i64,
    pub tag_id: i64,
}