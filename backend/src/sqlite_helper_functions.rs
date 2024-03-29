use crate::generated_types::authentication_message::Body as AuthBody;
use crate::generated_types::{AuthenticationMessage, Node, Secrets};
use prost::Message;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Connection, Result};
use std::env;
use std::sync::Arc;

extern crate bcrypt;

use bcrypt::{hash, verify, DEFAULT_COST};

extern crate colored;

use colored::*;

pub fn setup_sqlite_db() -> Result<()> {
    println!("Setting up SQLite database...");

    // Get the environmental variables required:
    let key = "SQLITE_FILE_LOCATION";

    println!("Retrieving environmental variable for SQLite location...");
    let sqlite_location = env::var(key).unwrap();

    println!(
        "Opening connection to SQLite at location: {}",
        sqlite_location
    );
    let conn = Connection::open(sqlite_location)?;

    println!("Creating nodes table...");
    create_nodes_table(&conn)?;

    println!("SQLite DB setup complete.");
    Ok(())
}

pub fn setup_sqlite_db_auth(sqlite_location: &str) -> Result<()> {
    println!("Setting up SQLite database...");

    // Get the environmental variables required:
    // let key = "SQLITE_FILE_LOCATION";

    // println!("Retrieving environmental variable for SQLite location...");
    // let sqlite_location = env::var(key).unwrap();

    // let sqlite_location = "auth.db";

    println!(
        "Opening connection to SQLite at location: {}",
        sqlite_location
    );
    let conn = Connection::open(sqlite_location)?;

    println!("Creating pass table...");
    create_pass_table(&conn)?;

    println!("SQLite DB setup complete.");
    Ok(())
}

pub fn create_pass_table(conn: &Connection) -> Result<()> {
    println!("Executing statement to create pass table if it does not exist...");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS pass (
            email TEXT PRIMARY KEY,
            hashpass TEXT
        )",
        [],
    )?;
    println!("Pass table created successfully.");
    Ok(())
}

pub fn authorized(
    pool: &Arc<Pool<SqliteConnectionManager>>,
    auth_message: AuthenticationMessage,
) -> Result<bool, String> {
    println!("Checking if hashed password is in db...");
    let connection = pool.get().expect("Failed to get connection from pool");
    println!("Connection obtained from pool successfully.");

    match auth_message.body.unwrap().clone() {
        AuthBody::Secrets(secret) => {
            println!("Secrets message received.");
            // let email = auth_message.email.unwrap();
            // let password = auth_message.password.unwrap();
            println!("Email: {}", secret.email);
            println!("Password len: {}", secret.password.len());
            let mut stmt = connection
                .prepare("SELECT hashpass FROM pass WHERE email = ?1")
                .unwrap();
            let mut rows = stmt.query(params![secret.email]).unwrap();
            let row = rows
                .next()
                .unwrap()
                .ok_or("Error getting rows".to_string())
                .unwrap();
            let hashpass: String = row.get(0).unwrap();
            println!("Hashpass: {}", hashpass);
            match verify(secret.password, &hashpass) {
                Ok(res) => {
                    println!("Password verified. {}.", res);
                    Ok(res)
                }
                Err(err) => {
                    println!("Password not verified.");
                    Err(format!("Password not verified: {:?}", err).to_string())
                }
            }
        }
        _ => Err("No secret included in auth message".to_string()),
    }
}

pub fn insert_user(
    pool: &Arc<Pool<SqliteConnectionManager>>,
    auth_message: AuthenticationMessage,
) -> Result<(), String> {
    println!("Inserting a user...");
    let connection = pool.get().expect("Failed to get connection from pool");
    println!("Connection obtained from pool successfully.");

    match auth_message.body.unwrap().clone() {
        AuthBody::Secrets(secret) => {
            println!("Secrets message received.");
            // let email = auth_message.email.unwrap();
            // let password = auth_message.password.unwrap();
            println!("Email: {}", secret.email);
            println!("Password: {}", secret.password);
            println!("Hashing password...");
            let hashed_pass = hash(secret.password, DEFAULT_COST).unwrap();
            println!("Hashed password: {}", hashed_pass);
            println!("Inserting hashed password into the database...");
            match connection.execute(
                "INSERT INTO pass (email, hashpass) VALUES (?1, ?2)",
                params![secret.email, hashed_pass],
            ) {
                Ok(_) => {
                    println!("Password insertion successful.");
                    Ok(())
                }
                Err(err) => {
                    println!("{}: {:?}", "Unable to insert password into db:".red(), err);
                    Err("Unable to insert password into db.".to_string())
                }
            }
        }
        _ => Err("No secret included in message.".to_string()),
    }
}

pub fn check_if_user_exists(
    pool: &Arc<Pool<SqliteConnectionManager>>,
    auth_message: AuthenticationMessage,
) -> Result<bool, String> {
    println!("Checking if user exists...");
    let connection = pool.get().expect("Failed to get connection from pool");
    println!("Connection obtained from pool successfully.");

    match auth_message.body.unwrap().clone() {
        AuthBody::Secrets(secret) => {
            println!("Secrets message received.");
            // let email = auth_message.email.unwrap();
            // let password = auth_message.password.unwrap();
            println!("Email: {}", secret.email);
            // println!("Password: {}", password);
            let mut stmt = connection
                .prepare("SELECT count(0) FROM pass WHERE email = ?1")
                .unwrap();

            // if there is a row, then the user exists
            let mut rows = stmt.query(params![secret.email]).unwrap();
            let row = rows
                .next()
                .unwrap()
                .ok_or("Error getting rows".to_string())
                .unwrap();
            let count: i64 = row.get(0).unwrap();
            if count > 0 {
                println!("User exists.");
                Ok(true)
            } else {
                println!("User does not exist.");
                Ok(false)
            }
        }
        _ => {
            // No string found
            Err("No secret included in message.".to_string())
        }
    }
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
        [],
    )?;
    println!("Nodes table created successfully.");
    Ok(())
}

pub fn insert_node(pool: Arc<Pool<SqliteConnectionManager>>, node: Node) -> Result<()> {
    println!("Inserting a node...");
    let connection = pool.get().expect("Failed to get connection from pool");
    println!("Connection obtained from pool successfully.");

    //ensure that the node has id information:
    // First check that it doesn't already have an id:

    println!("Serializing node...");
    let mut serialized_node = vec![];
    match node.clone().encode(&mut serialized_node) {
        Ok(_) => {
            println!("Serialization successful.");
            let id = node.node_info.clone().unwrap().id;
            let name = node.node_info.unwrap().name;
            let node_type = node.node_type;
            println!("Inserting serialized node into the database...");
            match
                connection.execute(
                    "INSERT OR REPLACE INTO nodes (id, name, type_name, serialized_node) VALUES (?1, ?2, ?3, ?4)",
                    params![id, name, node_type, serialized_node]
                )
            {
                Ok(_) => {
                    println!("Node insertion successful.");
                    Ok(())
                }
                Err(err) => {
                    println!("{}: {:?}", "Unable to insert node into db:".red(), err);
                    Ok(())
                }
            }
        }
        Err(err) => {
            println!("Unable to serialize node{:?}", err);
            Ok(())
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
            match connection.execute(
                "UPDATE nodes SET name = ?1, serialized_node = ?2 WHERE id = ?3",
                params![name, serialized_node, id],
            ) {
                Ok(count) => {
                    if count > 0 {
                        println!("Node updated successfully");
                        Ok(())
                    } else {
                        println!("No node found with the given ID");
                        Ok(())
                    }
                }
                Err(err) => {
                    println!("Unable to update node in db: {:?}", err);
                    Ok(())
                }
            }
        }
        Err(err) => {
            println!("Unable to serialize node: {:?}", err);
            Ok(())
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
