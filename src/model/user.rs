use chrono::{DateTime, Utc};
use rusqlite::params;
use serde::{Deserialize, Serialize};

use crate::db::new_conn;

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

pub fn add_user(user: &User) -> anyhow::Result<()> {
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
