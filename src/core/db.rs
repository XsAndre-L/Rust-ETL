use rusqlite::{Connection, Result};

// Change the return type to Result<Connection>
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
