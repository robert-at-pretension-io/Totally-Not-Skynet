use env_logger;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::env;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
mod env_vars_checker;
mod graph;
mod mongo;
mod openai;
mod receive_send;
mod settings;
mod sqlite_helper_functions;
mod websocket;

#[allow(non_snake_case)]
pub mod generated_types {
    include!(concat!(env!("OUT_DIR"), "/skynet.types.rs"));
}

use crate::receive_send::start_message_sending_loop;
use crate::websocket::start_websocket_server;
use colored::*;
use reqwest;

use once_cell::sync::OnceCell;

use crate::generated_types::Identity;

use crate::generated_types::authentication_message::Body::Secrets;

static SERVER_IDENTITY: OnceCell<Identity> = OnceCell::new();

#[tokio::main]
async fn main() {
    env_logger::init();

    let res = reqwest::get("http://api.ipify.org")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    println!("My external IP address is: {}", res);

    let server_identity: Identity = Identity {
        id: uuid::Uuid::new_v4().to_string(),
        ip_address: res.clone(),
    };

    println!("Server identity: {:?}", server_identity);
    // This is pretty sick actually. We can now access the server identity from anywhere in the codebase as a constant
    SERVER_IDENTITY.set(server_identity.clone()).unwrap();

    // Check that the environmental variables are set:
    let file_location = "./req_env_vars.txt";
    match env_vars_checker::check_env_vars(file_location) {
        Ok(_) => println!("Checked all environment variables."),
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("env variables not set");
        }
    }

    println!(
        "{}",
        "Need to re-enable check when changing environments back".red()
    );

    // Setup the db:
    match sqlite_helper_functions::setup_sqlite_db() {
        Ok(_) => {
            println!("sqlite working.. Tables are setup!");
        }
        Err(err) => {
            panic!("Oh goodness... {:?}", err);
        }
    }

    match sqlite_helper_functions::setup_sqlite_db_auth("auth.db") {
        Ok(_) => {
            println!("sqlite working.. Tables are setup!");
        }
        Err(err) => {
            panic!("Oh goodness... {:?}", err);
        }
    }

    let key = "SQLITE_FILE_LOCATION";
    let sqlite_location = env::var(key).unwrap();

    let manager = SqliteConnectionManager::file(sqlite_location);
    let pool = match Pool::new(manager) {
        Ok(p) => p,
        Err(err) => {
            panic!("Failed to create SQLite connection pool: {:?}", err);
        }
    };

    // make auth db pool
    let sqlite_location_auth = "auth.db";
    let manager_auth = SqliteConnectionManager::file(sqlite_location_auth);
    let pool_auth = match Pool::new(manager_auth) {
        Ok(p) => p,
        Err(err) => {
            panic!("Failed to create SQLite auth connection pool: {:?}", err);
        }
    };

    let (tx, rx) = mpsc::unbounded_channel();
    let rx = Arc::new(Mutex::new(rx));

    let (client_tx, client_rx) = mpsc::channel(100);

    // Spawn the WebSocket server task
    let server_task = tokio::spawn(async move {
        start_websocket_server(rx.clone(), client_tx).await;
    });

    let arc_pool = Arc::new(pool.clone());
    let auth_arc_pool = Arc::new(pool_auth.clone());

    // Spawn the message sender task
    let sender_task = tokio::spawn(async move {
        start_message_sending_loop(tx, client_rx, arc_pool, auth_arc_pool).await;
    });

    // Wait for both tasks to complete
    match tokio::join!(server_task, sender_task) {
        (Ok(_), Ok(_)) => {}
        (Err(e), _) => {
            println!("Error in server task: {:?}", e);
        }
        (_, Err(e)) => {
            println!("Error in sender task: {:?}", e);
        }
    }
}
