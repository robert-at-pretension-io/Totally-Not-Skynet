use futures_util::{SinkExt, StreamExt};

use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;

use bollard::Docker;

mod openai;
mod mongo;
mod utils;
mod protocol;
mod domain;
mod websocket;
mod settings;
mod send;


use crate::websocket::start_websocket_server;
use crate::send::start_message_sending_loop;





// use bollard::container::{CreateExecOptions, StartExecResults};

#[tokio::main]
async fn main() {
    // setup docker client

    let (tx, rx) = mpsc::unbounded_channel();
    let rx = Arc::new(Mutex::new(rx));

    let (client_tx, client_rx) = mpsc::channel(100);

    // Spawn the WebSocket server task
    let server_task = tokio::spawn(async move {
        start_websocket_server(rx.clone(), client_tx).await;
    });

    // Spawn the message sender task
    let sender_task = tokio::spawn(async move {
        start_message_sending_loop(tx, client_rx).await;
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
