use crate::generated_types::Node;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{ params, Connection, Result };
use std::env;
use std::sync::Arc;
use prost::Message;

extern crate colored;

use colored::*;

pub fn setup_sqlite_db() -> Result<()> {
    println!("Setting up SQLite database...");

    // Get the environmental variables required:
    let key = "SQLITE_FILE_LOCATION";

    println!("Retrieving environmental variable for SQLite location...");
    let sqlite_location = env::var(key).unwrap();

    println!("Opening connection to SQLite at location: {}", sqlite_location);
    let conn = Connection::open(sqlite_location)?;

    println!("Creating nodes table...");
    create_nodes_table(&conn)?;

    println!("SQLite DB setup complete.");
    return Ok(());
}

pub fn create_nodes_table(conn: &Connection) -> Result<()> {
    println!("Executing statement to create nodes table if it does not exist...");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS nodes (
            id TEXT PRIMARY KEY,
            name TEXT,
            type_name TEXT,
            serialized_node BLOB
        )",
        []
    )?;
    println!("Nodes table created successfully.");
    Ok(())
}

pub fn insert_node(pool: Arc<Pool<SqliteConnectionManager>>, node: Node) -> Result<()> {
    println!("Inserting a node...");
    let connection = pool.get().expect("Failed to get connection from pool");
    println!("Connection obtained from pool successfully.");

    println!("Serializing node...");
    let mut serialized_node = vec![];
    match node.clone().encode(&mut serialized_node) {
        Ok(_) => {
            println!("Serialization successful.");
            let id = node.node_info.clone().unwrap().id;
            let name = node.node_info.unwrap().name;
            let node_type = node.node_type.clone();
            println!("Inserting serialized node into the database...");
            match
                connection.execute(
                    "INSERT OR REPLACE INTO nodes (id, name, type_name, serialized_node) VALUES (?1, ?2, ?3, ?4)",
                    params![id, name, node_type, serialized_node]
                )
            {
                Ok(_) => {
                    println!("Node insertion successful.");
                    return Ok(());
                }
                Err(err) => {
                    println!("{}: {:?}", "Unable to insert node into db:".red(), err);
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
    println!("Updating a node...");
    let connection = pool.get().expect("Failed to get connection from pool");
    println!("Connection obtained from pool successfully.");

    println!("Serializing node for update...");
    let mut serialized_node = vec![];
    match node.clone().encode(&mut serialized_node) {
        Ok(_) => {
            println!("Serialization successful.");
            let id = node.node_info.clone().unwrap().id;
            let name = node.node_info.clone().unwrap().name;
            println!("Updating node in the database...");
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
    println!("Attempting to retrieve all nodes...");
    let connection = pool.get().expect("Failed to get connection from pool");
    println!("Connection retrieved from pool successfully.");
    println!("Preparing SQL statement for fetching all nodes...");
    let mut stmt = connection.prepare("SELECT serialized_node FROM nodes")?;
    println!("Statement prepared successfully.");

    println!("Querying database and deserializing nodes...");
    let node_iter = stmt.query_map([], |row| {
        let blob_data: Vec<u8> = row.get(0)?;
        let node = Node::decode(blob_data.as_slice()).expect("Failed to deserialize node");
        Ok(node)
    })?;

    let mut nodes = Vec::new();
    for node in node_iter {
        nodes.push(node?);
    }

    println!("All {:?} node(s) retrieved successfully.", nodes.len());
    Ok(nodes)
}
