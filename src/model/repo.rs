use std::str::FromStr;

use chrono::{DateTime, Utc};
use rusqlite::params;
use salvo::{writing::Json, Scribe};
use serde::{Deserialize, Serialize};

use crate::{
    db::new_conn,
    error::{ServiceError, ServiceResult},
};

#[derive(Debug)]
pub enum RepoStatus {
    Normal,
    Deleted,
}

impl RepoStatus {
    pub fn is_normal(&self) -> bool {
        matches!(self, Self::Normal)
    }
}

impl FromStr for RepoStatus {
    type Err = ServiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(Self::Normal),
            "deleted" => Ok(Self::Deleted),
            _ => Err(ServiceError::InternalServerError(
                "invalid repo status".to_owned(),
            )),
        }
    }
}

impl std::fmt::Display for RepoStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            Self::Normal => "normal",
            Self::Deleted => "deleted",
        };
        write!(f, "{}", status)
    }
}

#[derive(Debug)]
pub struct Repo {
    pub id: String,
    pub name: String,
    pub owner: String, // owner user_id
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status: RepoStatus,
}

pub fn add_repo(repo: &Repo) -> ServiceResult<()> {
    let conn = new_conn()?;
    conn.execute(
        "INSERT INTO repo (id, name, owner, description, created_at, updated_at, status) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            repo.id,
            repo.name,
            repo.owner,
            repo.description,
            repo.created_at,
            repo.updated_at,
            repo.status.to_string(),
        ],
    )?;
    Ok(())
}

pub fn update_repo(repo: &Repo) -> ServiceResult<()> {
    let conn = new_conn()?;
    conn.execute(
        "UPDATE repo SET name = ?2, description = ?3, updated_at = ?4, status = ?5 WHERE id = ?1",
        params![
            repo.id,
            repo.name,
            repo.description,
            repo.updated_at,
            repo.status.to_string(),
        ],
    )?;
    Ok(())
}

pub fn list_repos_by_owner_id(owner_id: &str) -> ServiceResult<Vec<Repo>> {
    let conn = new_conn()?;
    let mut stmt = conn.prepare("SELECT id, name, owner, description, created_at, updated_at, status FROM repo WHERE owner = ?1")?;
    let mut rows = stmt.query(params![owner_id])?;
    let mut repos = Vec::new();
    while let Some(row) = rows.next()? {
        let repo = Repo {
            id: row.get(0)?,
            name: row.get(1)?,
            owner: row.get(2)?,
            description: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
            status: RepoStatus::from_str(&row.get::<_, String>(6)?)?,
        };
        if repo.status.is_normal() {
            repos.push(repo);
        }
    }
    Ok(repos)
}

pub fn get_repo_by_id(repo_id: &str) -> ServiceResult<Option<Repo>> {
    let conn = new_conn()?;
    let mut stmt = conn.prepare("SELECT id, name, owner, description, created_at, updated_at, status FROM repo WHERE id = ?1")?;
    let mut rows = stmt.query(params![repo_id])?;
    let row = rows.next()?;
    match row {
        Some(row) => {
            let repo = Repo {
                id: row.get(0)?,
                name: row.get(1)?,
                owner: row.get(2)?,
                description: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
                status: RepoStatus::from_str(&row.get::<_, String>(6)?)?,
            };
            Ok(repo.status.is_normal().then_some(repo))
        }
        None => Ok(None),
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenApiPushRepoRequest {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<OpenApiPushRepoRequest> for Repo {
    fn from(value: OpenApiPushRepoRequest) -> Self {
        Self {
            id: value.id,
            name: value.name,
            owner: value.owner,
            description: value.description,
            created_at: value.created_at,
            updated_at: value.updated_at,
            status: RepoStatus::Normal,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenApiGetRepoResponse {
    pub id: String,
    pub name: String,
    pub owner: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenApiListRepoResponse(pub Vec<OpenApiGetRepoResponse>);

impl From<Repo> for OpenApiGetRepoResponse {
    fn from(repo: Repo) -> Self {
        Self {
            id: repo.id,
            name: repo.name,
            owner: repo.owner,
            description: repo.description,
            created_at: repo.created_at,
            updated_at: repo.updated_at,
        }
    }
}

impl Scribe for OpenApiGetRepoResponse {
    fn render(self, res: &mut salvo::Response) {
        res.render(Json(&self));
    }
}

impl Scribe for OpenApiListRepoResponse {
    fn render(self, res: &mut salvo::Response) {
        res.render(Json(&self));
    }
}
