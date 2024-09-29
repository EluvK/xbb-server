use rusqlite::Connection;

pub fn new_conn() -> anyhow::Result<Connection> {
    Ok(Connection::open("xbb.db3")?)
}

pub fn init_db() -> anyhow::Result<()> {
    let conn = new_conn()?;
    conn.execute_batch(include_str!("../sql/init_db.sql"))?;
    Ok(())
}
