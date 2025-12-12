use rusqlite::{Connection, Result, params};

// 2. Change the return type to Result<Connection>
pub fn setup_db(path: &str) -> Result<Connection> {
    let connection = Connection::open(path)?;

    connection.execute(
        "CREATE TABLE IF NOT EXISTS metrics (
            id TEXT PRIMARY KEY,
            timestamp INTEGER NOT NULL,
            value REAL NOT NULL,
            tag TEXT NOT NULL,
            positive INTEGER NOT NULL
        )",
        [],
    )?;

    Ok(connection)
}

// 3. Change return type to Result<()>
pub fn insert_record(
    connection: &Connection,
    id: &str,
    timestamp: i64,
    value: f64,
    tag: &str,
    pos: i32,
) -> Result<()> {
    connection.execute(
        "INSERT OR REPLACE INTO metrics (id, timestamp, value, tag, positive) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![id, timestamp, value, tag, pos],
    )?;
    Ok(())
}
