use chrono::{DateTime, Utc};
use rusqlite::params;
use salvo::{writing::Json, Scribe};
use serde::{Deserialize, Serialize};

use crate::{db::new_conn, error::ServiceResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: String,
    pub post_id: String,
    pub repo_id: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: String,
    pub parent_id: Option<String>,
}
