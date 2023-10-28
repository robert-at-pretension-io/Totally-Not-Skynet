use crate::generated_types::Node;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{ params, Connection, Result };
use std::env;
use std::sync::Arc;

pub fn setup_sqlite_db() -> Result<()> {
    // Get the environmental variables required:
    let key = "SQLITE_FILE_LOCATION";

    let sqlite_location = env::var(key).unwrap();

    let conn = Connection::open(sqlite_location)?;

    create_nodes_table(&conn)?;

    return Ok(());
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
pub fn insert_node(pool: Arc<Pool<SqliteConnectionManager>>, node: Node) -> Result<()> {
    let connection = pool.get().expect("Failed to get connection from pool");

    let mut serialized_node = vec![];
    match node.clone().encode(&mut serialized_node) {
        Ok(_) => {
            let id = node.node_info.clone().unwrap().id;
            let name = node.node_info.unwrap().name;
            match
                connection.execute(
                    "INSERT OR REPLACE INTO nodes (id, name, type_name, serialized_node) VALUES (?1, ?2, ?3, ?4)",
                    params![id, name, serialized_node]
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

pub fn update_node(pool: Arc<Pool<SqliteConnectionManager>>, node: &Node) -> Result<()> {
    let connection = pool.get().expect("Failed to get connection from pool");

    let mut serialized_node = vec![];
    match node.clone().encode(&mut serialized_node) {
        Ok(_) => {
            let id = node.node_info.clone().unwrap().id;
            let name = node.node_info.clone().unwrap().name;
            match
                connection.execute(
                    "UPDATE nodes SET name = ?1, serialized_node = ?2 WHERE id = ?3",
                    params![name, serialized_node, id]
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

pub fn fetch_all_nodes(pool: Arc<Pool<SqliteConnectionManager>>) -> Result<Vec<Node>> {
    let connection = pool.get().expect("Failed to get connection from pool");
    let mut stmt = connection.prepare("SELECT serialized_node FROM nodes")?;

    let node_iter = stmt.query_map([], |row| {
        let blob_data: Vec<u8> = row.get(0)?;
        let node = Node::decode(blob_data.as_slice()).expect("Failed to deserialize node");
        Ok(node)
    })?;

    let mut nodes = Vec::new();
    for node in node_iter {
        nodes.push(node?);
    }

    Ok(nodes)
}
