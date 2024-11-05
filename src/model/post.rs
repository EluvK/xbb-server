use chrono::{DateTime, Utc};
use rusqlite::params;
use salvo::{writing::Json, Scribe};
use serde::{Deserialize, Serialize};

use crate::{db::new_conn, error::ServiceResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub category: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: String,
    pub repo_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostSummary {
    pub id: String,
    pub title: String,
    pub category: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Post> for PostSummary {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            title: post.title,
            category: post.category,
            created_at: post.created_at,
            updated_at: post.updated_at,
        }
    }
}

pub fn add_post(post: &Post) -> ServiceResult<()> {
    let conn = new_conn()?;
    conn.execute(
        "INSERT INTO post (id, title, category, content, created_at, updated_at, author, repo_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            post.id,
            post.title,
            post.category,
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
            category: row.get(2)?,
            content: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
            author: row.get(6)?,
            repo_id: row.get(7)?,
        };
        posts.push(post);
    }
    Ok(posts)
}

pub fn get_post_by_id(id: &str) -> ServiceResult<Option<Post>> {
    let conn = new_conn()?;
    let mut stmt = conn.prepare("SELECT * FROM post WHERE id = ?1")?;
    let mut rows = stmt.query(params![id])?;
    let row = rows.next()?;
    match row {
        Some(row) => {
            let post = Post {
                id: row.get(0)?,
                title: row.get(1)?,
                category: row.get(2)?,
                content: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
                author: row.get(6)?,
                repo_id: row.get(7)?,
            };
            Ok(Some(post))
        }
        None => Ok(None),
    }
}

pub fn update_post(post: &Post) -> ServiceResult<()> {
    let conn = new_conn()?;
    conn.execute(
        "UPDATE post SET title = ?1, category = ?2, content = ?3, updated_at = ?4, repo_id = ?5 WHERE id = ?6",
        params![
            post.title,
            post.category,
            post.content,
            post.updated_at,
            post.repo_id,
            post.id
        ],
    )?;
    Ok(())
}

pub fn erase_post(id: &str) -> ServiceResult<()> {
    let conn = new_conn()?;
    conn.execute("DELETE FROM post WHERE id = ?1", params![id])?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenApiPushPostRequest {
    pub id: String,
    pub category: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub author: String,
    pub repo_id: String,
}

impl From<OpenApiPushPostRequest> for Post {
    fn from(value: OpenApiPushPostRequest) -> Self {
        Self {
            id: value.id,
            title: value.title,
            category: value.category,
            content: value.content,
            created_at: value.created_at,
            updated_at: value.updated_at,
            author: value.author,
            repo_id: value.repo_id,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiGetPostResponse {
    pub id: String,
    pub title: String,
    pub category: String,
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
            category: post.category,
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
