use std::sync::Arc;
use tokio::sync::{ mpsc, Mutex };

mod domain;
mod mongo;
mod openai;
mod receive_send;
mod settings;
mod utils;
mod websocket;
mod env_vars_checker;
mod check_if_docker_installed;

use crate::receive_send::start_message_sending_loop;
use crate::websocket::start_websocket_server;

// use bollard::container::{CreateExecOptions, StartExecResults};

#[tokio::main]
async fn main() {
    // setup docker client

    // Check that the environmental variables are set:
    let file_location = "./req_env_vars.txt";
    match env_vars_checker::check_env_vars(file_location) {
        Ok(_) => println!("Checked all environment variables."),
        Err(e) => {
            eprintln!("Error: {}", e);
            panic!("env variables not set");
        }
    }

    match check_if_docker_installed::docker_check() {
        Ok(_) => {
            println!("Docker check was successful!");
        }
        Err(e) => {
            println!("Error occurred during Docker check: {}", e);
        }
    }

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
