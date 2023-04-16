use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::{mpsc, Mutex};
use tokio::time::{self, Duration};
use tokio_tungstenite::tungstenite::Message;
use walkdir::WalkDir;

// Needed for setting up the docker container
use bollard::container::{Config, RemoveContainerOptions};
use bollard::Docker;

use bollard::exec::{CreateExecOptions, StartExecResults};
use bollard::image::CreateImageOptions;
use futures_util::TryStreamExt;

const IMAGE: &str = "alpine:3";

#[derive(Debug, Serialize, Deserialize)]
struct Action {
    prompt: String,
    name: String,
    system: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Process {
    name: String,
    trigger: String,
    triggers_next_process: String,
    waits_for_branch_completion: String,
    steps: Vec<String>,
    description: String,
    creates_process_branch: String,
    branch_step: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Identity {
    name: String,
}

impl Identity {
    fn new(name: String) -> Identity {
        Identity { name }
    }
    fn check_equal_to_string(&self, name: String) -> bool {
        self.name == name
    }
}

use serde_json::Result;
use std::str::FromStr;

// Define the Message trait
pub trait SystemMessage: Serialize + Deserialize<'static> {}

// Implement the Message trait for any type that implements Serialize and Deserialize
impl<T> SystemMessage for T where T: Serialize + Deserialize<'static> {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Goal {
    text: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct InitializeProject {
    initial_message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageTypes {
    Goal(Goal),
    InitializeProject(InitializeProject), // Add more types here
}

pub fn parse_message(message_str: &str) -> Option<MessageTypes> {
    if let Ok(msg) = serde_json::from_str::<Goal>(message_str) {
        return Some(MessageTypes::Goal(msg));
    }

    if let Ok(msg) = serde_json::from_str::<InitializeProject>(message_str) {
        return Some(MessageTypes::InitializeProject(msg));
    }

    None
}

use uuid::Uuid;

async fn start_websocket_server(
    rx: Arc<Mutex<mpsc::Receiver<(Identity, Message)>>>,
    client_tx: mpsc::Sender<(Identity, String)>,
) {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    while let Ok((stream, addr)) = listener.accept().await {
        let rx = rx.clone();
        let client_tx = client_tx.clone();
        // Spawn a new task for each incoming connection
        tokio::spawn(async move {
            let id = Uuid::new_v4();

            let mut this_client = Identity {
                name: id.to_string(),
            };
            let this_client_clone = this_client.clone();

            let ws_stream = match tokio_tungstenite::accept_async(stream).await {
                Ok(ws_stream) => ws_stream,
                Err(e) => {
                    println!("Error during the websocket handshake occurred: {:?}", e);
                    return;
                }
            };
            println!("WebSocket connection established: {}", addr);

            let (mut outgoing, mut incoming) = ws_stream.split();

            let cloned_client = this_client.clone();

            // Send/receive actions, processes, and messages by reading from the channel
            tokio::spawn(async move {
                while let Some(outgoing_msg) = rx.lock().await.recv().await {
                    // check if the client has the same identity as the message
                    if outgoing_msg.0.name == cloned_client.name {
                        outgoing.send(outgoing_msg.1.clone()).await.unwrap();
                    }
                }
            });

            // Process incoming messages for each connection
            while let Some(msg) = incoming.next().await {
                match msg {
                    Ok(msg) => {
                        println!(
                            "Received a message from {}: {}",
                            addr,
                            msg.to_text().unwrap()
                        );

                        client_tx
                            .send((this_client.clone(), msg.to_string()))
                            .await
                            .unwrap();
                    }
                    Err(e) => {
                        println!("Error processing message from {}: {:?}", addr, e);
                        break;
                    }
                }
            }
        });
    }
}
use mongodb::{options::ClientOptions, Client};
async fn return_db() -> mongodb::Database {
    let client = Client::with_uri_str("mongodb://localhost:27017/")
        .await
        .unwrap();

    client.database("skynet")
}

type DockerId = String;

async fn start_message_sending_loop(
    docker: Docker,
    tx: mpsc::Sender<(Identity, Message)>,
    mut client_rx: mpsc::Receiver<(Identity, String)>,
    my_actions: Vec<Action>,
    my_processes: Vec<Process>,
) {
    let alpine_config = Config {
        image: Some(IMAGE),
        tty: Some(true),
        attach_stdout: Some(true),
        attach_stderr: Some(true),
        ..Default::default()
    };

    let mut identities: Vec<(Identity, DockerId)> = Vec::new();

    //read messages from the client
    while let Some(msg) = client_rx.recv().await {
        println!("Received a message from the client: {}", msg.1);

        let received_message: Option<MessageTypes> = parse_message(&msg.1);

        let message_contents: MessageTypes;

        if received_message.is_none() {
            continue;
        } else {
            message_contents = received_message.unwrap();
        }

        match message_contents {
            MessageTypes::Goal(_) => todo!(),
            MessageTypes::InitializeProject(_) => {
                // send the actions to the client
                for action in &my_actions {
                    tx.send((
                        Identity::new(msg.0.name.to_string()),
                        Message::Text(serde_json::to_string(&action).unwrap()),
                    ))
                    .await
                    .unwrap();
                }

                //send processes to the client
                for process in &my_processes {
                    tx.send((
                        Identity::new(msg.0.name.to_string()),
                        Message::Text(serde_json::to_string(&process).unwrap()),
                    ))
                    .await
                    .unwrap();
                }

                let id = docker
                    .create_container::<&str, &str>(None, alpine_config.clone())
                    .await
                    .unwrap()
                    .id;

                identities.push((msg.0, id));
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // get the database
    let db = return_db().await;
    let action_collection = db.collection::<Action>("actions");
    let process_collection = db.collection::<Process>("processes");

    let mut actions_cursor = action_collection.find(None, None).await.unwrap();

    let mut processes_cursor = process_collection.find(None, None).await.unwrap();

    let mut actions = Vec::new();
    let mut processes = Vec::new();

    while let Some(action) = actions_cursor.next().await {
        actions.push(action.unwrap());
    }

    while let Some(process) = processes_cursor.next().await {
        processes.push(process.unwrap());
    }
    // setup docker client
    let docker = Docker::connect_with_local_defaults().unwrap();

    docker
        .create_image(
            Some(CreateImageOptions {
                from_image: IMAGE,
                ..Default::default()
            }),
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    let (tx, rx) = mpsc::channel(100);
    let rx = Arc::new(Mutex::new(rx));

    let (client_tx, mut client_rx) = mpsc::channel(100);

    // Spawn the WebSocket server task
    let server_task = tokio::spawn(async move {
        start_websocket_server(rx.clone(), client_tx).await;
    });

    // Spawn the message sender task
    let sender_task = tokio::spawn(async move {
        start_message_sending_loop(docker, tx, client_rx, actions, processes).await;
    });

    // Wait for both tasks to complete
    tokio::join!(server_task, sender_task);
}