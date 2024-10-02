use chrono::{DateTime, Utc};
use rusqlite::params;
use salvo::{async_trait, writing::Json, Writer};
use serde::{Deserialize, Serialize};

use crate::{db::new_conn, error::ServiceResult};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub password: String,
    pub avatar_url: Option<String>,
}

impl User {
    pub fn new(name: String, password: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            password,
            avatar_url: None,
        }
    }
}

pub fn add_user(user: &User) -> ServiceResult<()> {
    let conn = new_conn()?;
    conn.execute(
        "INSERT INTO user (id, name, created_at, updated_at, password) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            user.id,
            user.name,
            user.created_at,
            user.updated_at,
            user.password
        ],
    )?;
    Ok(())
}

pub fn get_user_by_name(name: &str) -> ServiceResult<Option<User>> {
    let conn = new_conn()?;
    let mut stmt = conn.prepare("SELECT * FROM user WHERE name = ?1")?;
    let mut rows = stmt.query(params![name])?;
    let row = rows.next()?;
    match row {
        Some(row) => {
            let user = User {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
                updated_at: row.get(3)?,
                password: row.get(4)?,
                avatar_url: row.get(5)?,
            };
            Ok(Some(user))
        }
        None => Ok(None),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenApiNewUserRequest {
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenApiGetUserResponse {
    pub id: String,
    pub name: String,
    pub avatar_url: Option<String>,
}

#[async_trait]
impl Writer for OpenApiGetUserResponse {
    async fn write(
        self,
        _req: &mut salvo::Request,
        _depot: &mut salvo::Depot,
        res: &mut salvo::Response,
    ) {
        res.render(Json(&self));
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::model::user::{add_user, User};

    #[test]
    fn test_user() -> anyhow::Result<()> {
        let user = User::new("name".into(), "password".into());
        add_user(&user)?;
        Ok(())
    }
}
