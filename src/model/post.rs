use chrono::{DateTime, Utc};
use rusqlite::params;
use salvo::{writing::Json, Scribe};
use serde::{Deserialize, Serialize};

use crate::{
    db::new_conn,
    error::{ServiceError, ServiceResult},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: String,
    pub repo_id: String,
}

impl Post {
    pub fn new(title: String, content: String, author: String, repo_id: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            content,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            author,
            repo_id,
        }
    }
}

pub fn add_post(post: &Post) -> ServiceResult<()> {
    let conn = new_conn()?;
    conn.execute(
        "INSERT INTO post (id, title, content, created_at, updated_at, author, repo_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            post.id,
            post.title,
            post.content,
            post.created_at,
            post.updated_at,
            post.author,
            post.repo_id
        ],
    )?;
    Ok(())
}

pub fn list_posts_by_repo_id(repo_id: &str) -> ServiceResult<Vec<Post>> {
    let conn = new_conn()?;
    let mut stmt = conn.prepare("SELECT * FROM post WHERE repo_id = ?1")?;
    let mut rows = stmt.query(params![repo_id])?;
    let mut posts = Vec::new();
    while let Some(row) = rows.next()? {
        let post = Post {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
            author: row.get(5)?,
            repo_id: row.get(6)?,
        };
        posts.push(post);
    }
    Ok(posts)
}

pub fn get_post_by_id(id: &str) -> ServiceResult<Post> {
    let conn = new_conn()?;
    let mut stmt = conn.prepare("SELECT * FROM post WHERE id = ?1")?;
    let mut rows = stmt.query(params![id])?;
    let row = rows.next()?;
    match row {
        Some(row) => {
            let post = Post {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
                author: row.get(5)?,
                repo_id: row.get(6)?,
            };
            Ok(post)
        }
        None => Err(ServiceError::NotFound(format!("post {} not found", id))),
    }
}

pub fn update_post(post: &Post) -> ServiceResult<()> {
    let conn = new_conn()?;
    conn.execute(
        "UPDATE post SET title = ?1, content = ?2, updated_at = ?3 WHERE id = ?4",
        params![post.title, post.content, post.updated_at, post.id],
    )?;
    Ok(())
}

pub fn erase_post(id: &str) -> ServiceResult<()> {
    let conn = new_conn()?;
    conn.execute("DELETE FROM post WHERE id = ?1", params![id])?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiNewPostRequest {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiGetPostResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: String,
    pub repo_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiListPostResponse(pub Vec<OpenApiGetPostResponse>);

impl From<Post> for OpenApiGetPostResponse {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            title: post.title,
            content: post.content,
            created_at: post.created_at,
            updated_at: post.updated_at,
            author: post.author,
            repo_id: post.repo_id,
        }
    }
}

impl Scribe for OpenApiGetPostResponse {
    fn render(self, res: &mut salvo::Response) {
        res.render(Json(&self));
    }
}

impl Scribe for OpenApiListPostResponse {
    fn render(self, res: &mut salvo::Response) {
        res.render(Json(&self));
    }
}
