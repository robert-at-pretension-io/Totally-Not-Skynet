use rusqlite::{ Connection, Result, params };
use std::env;
use crate::generated_types::Node;

pub fn setup_sqlite() -> Result<Connection> {
    // Get the environmental variables required:
    let key = "SQLITE_FILE_LOCATION";

    let sqlite_location = env::var(key).unwrap();

    let conn = Connection::open(sqlite_location)?;

    create_nodes_table(&conn)?;

    return Ok(conn);
}

pub fn create_nodes_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS nodes (
            id TEXT PRIMARY KEY,
            name TEXT,
            type_name INTEGER,
            serialized_node BLOB
        )",
        []
    )?;
    Ok(())
}

pub fn insert_node(conn: &Connection, node: &Node) -> Result<()> {
    let mut serialized_node = vec![];
    node.encode(&mut serialized_node)?;

    conn.execute(
        "INSERT OR REPLACE INTO nodes (id, name, type_name, serialized_node) VALUES (?1, ?2, ?3, ?4)",
        params![node.id, node.name, node.type_name as i32, serialized_node]
    )?;

    Ok(())
}
