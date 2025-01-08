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

pub fn add_comment(comment: &Comment) -> ServiceResult<()> {
    let conn = new_conn()?;
    conn.execute(
        "INSERT INTO comment (id, post_id, repo_id, content, created_at, updated_at, author, parent_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            comment.id,
            comment.post_id,
            comment.repo_id,
            comment.content,
            comment.created_at,
            comment.updated_at,
            comment.author,
            comment.parent_id
        ],
    )?;
    Ok(())
}

pub fn get_comment_by_id(id: &str) -> ServiceResult<Option<Comment>> {
    let conn = new_conn()?;
    let mut stmt = conn.prepare("SELECT id, post_id, repo_id, content, created_at, updated_at, author, parent_id FROM comment WHERE id = ?1")?;
    let mut rows = stmt.query(params![id])?;
    if let Some(row) = rows.next()? {
        Ok(Some(Comment {
            id: row.get(0)?,
            post_id: row.get(1)?,
            repo_id: row.get(2)?,
            content: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
            author: row.get(6)?,
            parent_id: row.get(7)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn list_comments_by_post_id(post_id: &str) -> ServiceResult<Vec<Comment>> {
    let conn = new_conn()?;
    let mut stmt = conn.prepare("SELECT id, post_id, repo_id, content, created_at, updated_at, author, parent_id FROM comment WHERE post_id = ?1")?;
    let mut rows = stmt.query(params![post_id])?;
    let mut comments = Vec::new();
    while let Some(row) = rows.next()? {
        comments.push(Comment {
            id: row.get(0)?,
            post_id: row.get(1)?,
            repo_id: row.get(2)?,
            content: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
            author: row.get(6)?,
            parent_id: row.get(7)?,
        });
    }
    Ok(comments)
}

pub fn delete_comment_by_id(id: &str) -> ServiceResult<()> {
    let conn = new_conn()?;
    conn.execute("DELETE FROM comment WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn update_comment(comment: &Comment) -> ServiceResult<()> {
    let conn = new_conn()?;
    conn.execute(
        "UPDATE comment SET post_id = ?2, repo_id = ?3, content = ?4, updated_at = ?5, author = ?6, parent_id = ?7 WHERE id = ?1",
        params![
            comment.id,
            comment.post_id,
            comment.repo_id,
            comment.content,
            comment.updated_at,
            comment.author,
            comment.parent_id
        ],
    )?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenApiPushCommentRequest {
    pub id: Option<String>, // some for update, none for insert
    pub content: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenApiGetCommentResponse {
    pub id: String,
    pub post_id: String,
    pub repo_id: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: String,
    pub parent_id: Option<String>,
}

impl Scribe for OpenApiGetCommentResponse {
    fn render(self, res: &mut salvo::Response) {
        res.render(Json(&self));
    }
}

impl From<Comment> for OpenApiGetCommentResponse {
    fn from(value: Comment) -> Self {
        Self {
            id: value.id,
            post_id: value.post_id,
            repo_id: value.repo_id,
            content: value.content,
            created_at: value.created_at,
            updated_at: value.updated_at,
            author: value.author,
            parent_id: value.parent_id,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenApiCommentSummaryResponse {
    pub id: String,
    pub post_id: String,
    pub repo_id: String,
    pub updated_at: DateTime<Utc>,
}

impl From<Comment> for OpenApiCommentSummaryResponse {
    fn from(value: Comment) -> Self {
        Self {
            id: value.id,
            post_id: value.post_id,
            repo_id: value.repo_id,
            updated_at: value.updated_at,
        }
    }
}

impl Scribe for OpenApiCommentSummaryResponse {
    fn render(self, res: &mut salvo::Response) {
        res.render(Json(&self));
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiListCommentResponse(pub Vec<OpenApiGetCommentResponse>);

impl Scribe for OpenApiListCommentResponse {
    fn render(self, res: &mut salvo::Response) {
        res.render(Json(&self));
    }
}
