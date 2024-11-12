use rusqlite::params;

use crate::{db::new_conn, error::ServiceResult};

pub fn add_subscribe(user_id: &str, repo_id: &str) -> ServiceResult<()> {
    let id = uuid::Uuid::new_v4().to_string();
    let conn = new_conn()?;
    conn.execute(
        "INSERT INTO subscribe (id, user_id, repo_id) VALUES (?1, ?2, ?3)",
        params![id, user_id, repo_id],
    )?;
    Ok(())
}

pub fn check_subscribe(user_id: &str, repo_id: &str) -> ServiceResult<bool> {
    let conn = new_conn()?;
    let mut stmt =
        conn.prepare("SELECT COUNT(*) FROM subscribe WHERE user_id = ?1 AND repo_id = ?2")?;
    let count: i64 = stmt.query_row(params![user_id, repo_id], |row| row.get(0))?;
    Ok(count > 0)
}

pub fn fetch_subscribe(user_id: &str) -> ServiceResult<Vec<String>> {
    let conn = new_conn()?;
    let mut stmt = conn.prepare("SELECT * FROM subscribe WHERE user_id = ?1")?;
    let mut rows = stmt.query(params![user_id])?;
    let mut repos = Vec::new();
    while let Some(row) = rows.next()? {
        repos.push(row.get(2)?);
    }
    Ok(repos)
}

pub fn delete_subscribe(user_id: &str, repo_id: &str) -> ServiceResult<()> {
    let conn = new_conn()?;
    conn.execute(
        "DELETE FROM subscribe WHERE user_id = ?1 AND repo_id = ?2",
        params![user_id, repo_id],
    )?;
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     #[allow(unused_imports)]
//     use super::*;

//     #[test]
//     fn test_subscribe() {
//         let user_id = "1eec14bf-e9a0-4c73-8f50-a8681351ce89";
//         let repo_id = "cae646fe-12c4-42a3-bd92-19f49a22a8b4";
//         add_subscribe(user_id, repo_id).unwrap();
//         assert!(check_subscribe(user_id, repo_id).unwrap());
//         dbg!(&fetch_subscribe(user_id).unwrap());
//         delete_subscribe(user_id, repo_id).unwrap();
//         assert!(!check_subscribe(user_id, repo_id).unwrap());
//     }
// }

// CREATE TABLE IF NOT EXISTS "subscribe" (
//     "id" TEXT PRIMARY KEY,
//     "user_id" TEXT NOT NULL,
//     "repo_id" TEXT NOT NULL,
//     FOREIGN KEY("user_id") REFERENCES "user"("id"),
//     FOREIGN KEY("repo_id") REFERENCES "repo"("id")
// );
