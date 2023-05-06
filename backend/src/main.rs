use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::tungstenite::Message;

// Needed for setting up the docker container
// use bollard::container::{Config, RemoveContainerOptions};
// use bollard::Docker;

// use bollard::exec::{CreateExecOptions, StartExecResults};
// use bollard::image::CreateImageOptions;

const IMAGE: &str = "alpine:3";

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Action {
    prompt: String,
    name: String,
    system: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
pub struct OpenaiKey {
    key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Prompt {
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageTypes {
    Goal(Goal),
    InitializeProject(InitializeProject), // Add more types here
    SetOpenAIKey(OpenaiKey),
    GetTextCompletion(Prompt)
}

pub fn parse_message(message_str: &str) -> Option<MessageTypes> {
    if let Ok(msg) = serde_json::from_str::<Goal>(message_str) {
        return Some(MessageTypes::Goal(msg));
    }

    if let Ok(msg) = serde_json::from_str::<InitializeProject>(message_str) {
        return Some(MessageTypes::InitializeProject(msg));
    }

    if let Ok(msg) = serde_json::from_str::<OpenaiKey>(message_str) {
        return Some(MessageTypes::SetOpenAIKey(msg));
    }

    if let Ok(msg) = serde_json::from_str::<Prompt>(message_str) {
        return Some(MessageTypes::GetTextCompletion(msg));
    }

    None
}



// types used for sending messages to openai
#[derive(Debug, Deserialize, Serialize, Clone)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Choice {
    message: ChatMessage,
    finish_reason: String,
    index: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ChatCompletion {
    id: String,
    object: String,
    created: u64,
    model: String,
    usage: Usage,
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ChatMessage {
    role: Role,
    content: String,
}

impl fmt::Display for ChatMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.role, self.content)
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone, Copy)]
enum Role {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

impl fmt::Display for Role {
    // this is the implementation of the fmt::Display trait
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::System => write!(f, "system"),
            Role::User => write!(f, "user"),
            Role::Assistant => write!(f, "assistant"),
        }
    }
}


use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use serde_json::json;

use serde_json::{ Value as JsonValue};

// Your existing type definitions here...

async fn call_openai(messages: Vec<ChatMessage>, api_key: String) -> Result<String> {
    // Define the URL for the API endpoint
    let url = "https://api.openai.com/v1/chat/completions";

    // Define the initial request body
    let mut body: JsonValue = json!({
        "model": "gpt-3.5-turbo",
        "messages": messages,
        "temperature": 0.7
    });

    // Set up the headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap());

    // Create an HTTP client
    let client = reqwest::Client::new();

    let mut response_string = String::new();

    // Loop to make repeated API requests
    loop {
        // Make the HTTP POST request asynchronously
        let response = client
            .post(url)
            .headers(headers.clone())
            .body(body.to_string())
            .send()
            .await
            .unwrap();

        // Deserialize the response JSON into the ChatCompletion struct
        let chat_completion: ChatCompletion = serde_json::from_str(&response.text().await.unwrap())?;

        // Print the result
        println!("{:#?}", chat_completion);

        // Check if the finish_reason is "stop"
        if let Some(choice) = chat_completion.choices.first() {
            if choice.finish_reason == "stop" {
                // If the finish_reason is "stop", exit the loop
                response_string = choice.message.content.clone();
                break;
            } else {
                // If the finish_reason is not "stop", update the request body
                // to include the assistant's response and make another request
                if let JsonValue::Array(messages) = &mut body["messages"] {
                    messages.push(json!(choice.message));
                }
            }
        } else {
            // If there are no choices, exit the loop
            break;
        }
    }

    Ok(response_string)
}





use uuid::Uuid;

async fn start_websocket_server(
    rx: Arc<tokio::sync::Mutex<UnboundedReceiver<(Identity, Message)>>>,
    client_tx: mpsc::Sender<(Identity, String)>,
) {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    let mut request_dispatcher: HashMap<
        Identity,
        UnboundedSender<Message>,
    > = HashMap::new();
    let mut thread_safe_request_dispatcher = Arc::new(Mutex::new(request_dispatcher));
    //write two tasks:
    //

    let thread_safe_request_dispatcher_clone_1 = thread_safe_request_dispatcher.clone();

    //dispatch the message to the appropriate client
    tokio::spawn(async move {
        while let Some(outgoing_msg) = rx.lock().await.recv().await {
            println!(
                "\n\nSending message:\n {} \nto: {}",
                outgoing_msg.1, outgoing_msg.0.name
            );

            //get the client's outgoing channel
            let sending_channel = thread_safe_request_dispatcher_clone_1
                .lock()
                .await
                .get_mut(&outgoing_msg.0)
                .unwrap()
                .clone();
            sending_channel.send(outgoing_msg.1);

        }
    });
    let thread_safe_request_dispatcher_clone_2 = thread_safe_request_dispatcher.clone();

    while let Ok((stream, addr)) = listener.accept().await {
        // let rx = rx.clone();
        let client_tx = client_tx.clone();

        // create an unbounded sender and receiver
        let (local_tx, mut local_rx) = tokio::sync::mpsc::unbounded_channel::<Message>();

        let thread_safe_request_dispatcher_clone_3 = thread_safe_request_dispatcher.clone();

        // Spawn a new task for each incoming connection
        tokio::spawn(async move {
            let id = Uuid::new_v4();

            thread_safe_request_dispatcher_clone_3
                .lock()
                .await
                .insert(Identity::new(id.to_string()),  local_tx);

            let this_client = Identity {
                name: id.to_string(),
            };

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

            tokio::spawn(async move {
                while let Some(outgoing_msg) = local_rx.recv().await {
                    println!(
                        "\n\nSending message:\n {} \nto: {}",
                        outgoing_msg, cloned_client.name
                    );

                    match outgoing.send(outgoing_msg.clone()).await {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error sending message to client: {:?}", e);
                            break;
                        }
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

                        match client_tx.send((this_client.clone(), msg.to_string())).await {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Error sending message to client: {:?}", e);
                                break;
                            }
                        }
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
use mongodb::{Client};
async fn return_db() -> mongodb::Database {
    let client = Client::with_uri_str("mongodb://localhost:27017/")
        .await
        .unwrap();

    client.database("skynet")
}

type DockerId = String;

struct RuntimeSettings {
    openai_api_key : String
}

async fn start_message_sending_loop(
    // docker: Docker,
    tx: UnboundedSender<(Identity, Message)>,
    mut client_rx: mpsc::Receiver<(Identity, String)>,
) {

    // let alpine_config = Config {
    //     image: Some(IMAGE),
    //     tty: Some(true),
    //     attach_stdout: Some(true),
    //     attach_stderr: Some(true),
    //     ..Default::default()
    // };

    let mut docker_containers: Vec<(Identity, DockerId)> = Vec::new();

    let mut runtimeSettings : HashMap<Identity, RuntimeSettings> = HashMap::new();

    // get the database
    let db = return_db().await;

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
                // get the actions and processes from the db

                // send the actions to the client

                let (my_actions, my_processes) = get_actions_and_processes(&db).await;

                println!("Sending {} actions to the client", my_actions.len());

                for action in &my_actions.clone() {
                    // sleep(delay_duration).await;

                    println!("sending action {} to {}", action.name, msg.0.name);

                    match tx.send((
                        Identity::new(msg.0.name.to_string()),
                        Message::Text(serde_json::to_string(&action).unwrap()),
                    )) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error sending message to client: {:?}", e);
                            break;
                        }
                    }
                }

                println!("Sending {} processes to the client", my_processes.len());

                //send processes to the client
                for process in &my_processes.clone() {
                    // sleep(delay_duration).await;
                    match tx.send((
                        Identity::new(msg.0.name.to_string()),
                        Message::Text(serde_json::to_string(&process).unwrap()),
                    )) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error sending message to client: {:?}", e);
                            break;
                        }
                    }
                }

                // let id = docker
                //     .create_container::<&str, &str>(None, alpine_config.clone())
                //     .await
                //     .unwrap()
                //     .id;

                //     docker_containers.push((msg.0, id));
            }
            MessageTypes::SetOpenAIKey(key) => {
                println!("Setting openai key for {}", msg.0.name);
                runtimeSettings.insert(msg.0, RuntimeSettings{
                    openai_api_key: key.key,
                });
            }
            MessageTypes::GetTextCompletion(prompt) => {

                // check to see if the client has an openai key
                let openai_api_key = match runtimeSettings.get(&msg.0) {
                    Some(settings) => Some(settings.openai_api_key.clone()),
                    None => {
                        println!("No openai key set for {}", msg.0.name);
                        None
                        
                    }
                };

                if openai_api_key.is_some() {

                    let messages = vec!(ChatMessage {
                        role: Role::System,
                        content: "You are a helpful assistant, you will help the user in any way they ask.".to_string()
                    },
                    ChatMessage {
                        role: Role::User,
                        content: prompt.text.clone()
                    }
                );

                    let response = call_openai(messages, openai_api_key.unwrap()).await;
                    
                    match response {
                        Ok(res) => {
                            match tx.send((
                                Identity::new(msg.0.name.to_string()),
                                Message::Text(res),
                            )) {
                                Ok(_) => {}
                                Err(e) => {
                                    println!("Error sending message to client: {:?}", e);
                                    break;
                                }
                            }
                        },
                        Err(_) => todo!(),
                    }
                    
                }

                println!("Received text completion from {}", msg.0.name);

                

 
            }
        }
    }
}

async fn get_actions_and_processes(db: &mongodb::Database) -> (Vec<Action>, Vec<Process>) {
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

    (actions, processes)
}

#[tokio::main]
async fn main() {
    // setup docker client
    // let docker = Docker::connect_with_local_defaults().unwrap();

    // docker
    //     .create_image(
    //         Some(CreateImageOptions {
    //             from_image: IMAGE,
    //             ..Default::default()
    //         }),
    //         None,
    //         None,
    //     )
    //     .try_collect::<Vec<_>>()
    //     .await
    //     .unwrap();

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
