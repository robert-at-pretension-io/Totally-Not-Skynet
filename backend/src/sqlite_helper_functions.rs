use rusqlite::{ Connection, Result, params };
use std::env;
use crate::generated_types::Node;

pub fn get_sqlite_db() -> Result<Connection> {
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
            type_name TEXT,
            serialized_node BLOB
        )",
        []
    )?;
    Ok(())
}
use prost::Message;
pub fn insert_node(conn: &Connection, node: &Node) -> Result<()> {
    let mut serialized_node = vec![];
    match node.encode(&mut serialized_node) {
        Ok(_) => {
            match
                conn.execute(
                    "INSERT OR REPLACE INTO nodes (id, name, type_name, serialized_node) VALUES (?1, ?2, ?3, ?4)",
                    params![node.id, node.name, node.type_name, serialized_node]
                )
            {
                Ok(_) => {
                    return Ok(());
                }
                Err(err) => {
                    println!("Unable to insert node into db: {:?}", err);
                    return Ok(());
                }
            }
        }
        Err(err) => {
            println!("Unable to serialize node{:?}", err);
            return Ok(());
        }
    }
}

pub fn update_node(conn: &Connection, node: &Node) -> Result<()> {
    let mut serialized_node = vec![];
    match node.encode(&mut serialized_node) {
        Ok(_) => {
            match
                conn.execute(
                    "UPDATE nodes SET name = ?1, type_name = ?2, serialized_node = ?3 WHERE id = ?4",
                    params![node.name, node.type_name, serialized_node, node.id]
                )
            {
                Ok(count) => {
                    if count > 0 {
                        println!("Node updated successfully");
                        return Ok(());
                    } else {
                        println!("No node found with the given ID");
                        return Ok(());
                    }
                }
                Err(err) => {
                    println!("Unable to update node in db: {:?}", err);
                    return Ok(());
                }
            }
        }
        Err(err) => {
            println!("Unable to serialize node: {:?}", err);
            return Ok(());
        }
    }
}
