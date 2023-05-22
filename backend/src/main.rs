use bson::{doc, oid::ObjectId};
use futures_util::{SinkExt, StreamExt};
use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

// Needed for setting up the docker container
// use bollard::container::{Config, RemoveContainerOptions};
// use bollard::Docker;

// use bollard::exec::{CreateExecOptions, StartExecResults};
// use bollard::image::CreateImageOptions;

// const IMAGE: &str = "alpine:3";

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Action {
    _id: Option<ObjectId>,
    prompt: String,
    input_variables: Vec<String>,
    output_variables: Vec<String>,
    name: String,
    system: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Process {
    _id: Option<ObjectId>,
    name: String,
    graph: String,
    description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
struct Identity {
    name: String,
}

impl Identity {
    fn new(name: String) -> Identity {
        Identity { name }
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
pub struct UserSettings {
    openai_api_key: String,
    mongo_db_uri: String,
}

impl UserSettings {
    pub fn new() -> Option<UserSettings> {
        let openai_api_key = env::var("OPENAI_API_KEY").unwrap();
        let mongo_db_uri = env::var("MONGO_DB_URI").unwrap();

        Some(UserSettings {
            openai_api_key,
            mongo_db_uri,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Prompt {
    prompt_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateAction {
    action: Action,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAction {
    create_action: Action,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateProcess {
    create_process: Process,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageTypes {
    Goal(Goal),
    InitializeProject(InitializeProject), // Add more types here
    SetUserSettings(UserSettings),
    GetTextCompletion(Prompt),
    UpdateAction(UpdateAction),
    CreateAction(CreateAction),
    CreateProcess(CreateProcess),
}

pub fn parse_message(message_str: &str) -> Option<MessageTypes> {
    use serde_json::Value;
    let value: Value = match serde_json::from_str(message_str) {
        Ok(val) => val,
        Err(_) => return None, // or handle this error as you see fit
    };

    if let Some(obj) = value.as_object() {
        if let Some(create_action_value) = obj.get("create_action") {
            if let Some(create_action_obj) = create_action_value.as_object() {
                let action = Action {
                    _id: None, // Assuming you have changed your struct field to `_id`
                    input_variables: create_action_obj
                        .get("input_variables")
                        .and_then(|v| v.as_array())
                        .unwrap_or(&vec![])
                        .iter()
                        .map(|v| v.as_str().unwrap_or("").to_string())
                        .collect(),
                    output_variables: create_action_obj
                        .get("output_variables")
                        .and_then(|v| v.as_array())
                        .unwrap_or(&vec![])
                        .iter()
                        .map(|v| v.as_str().unwrap_or("").to_string())
                        .collect(),
                    name: create_action_obj
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    prompt: create_action_obj
                        .get("prompt")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    system: create_action_obj
                        .get("system")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                };
                return Some(MessageTypes::CreateAction(CreateAction {
                    create_action: action,
                }));
            }
        }
        if let Some(create_process_value) = obj.get("create_process") {
            if let Some(create_process_obj) = create_process_value.as_object() {
                let graph = match create_process_obj.get("graph") {
                    Some(v) => match serde_json::to_string(v) {
                        Ok(s) => s,
                        Err(_) => "".to_string(),
                    },
                    None => "".to_string(),
                };
                let process = Process {
                    _id: None, // Assuming you have changed your struct field to `_id`
                    name: create_process_obj
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    graph,
                    description: create_process_obj
                        .get("description")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                };
                println!("create_process: {:?}", process);
                return Some(MessageTypes::CreateProcess(CreateProcess {
                    create_process: process,
                }));
            }
        }
    }

    if let Ok(msg) = serde_json::from_str::<Goal>(message_str) {
        return Some(MessageTypes::Goal(msg));
    }

    if let Ok(msg) = serde_json::from_str::<InitializeProject>(message_str) {
        return Some(MessageTypes::InitializeProject(msg));
    }

    if let Ok(msg) = serde_json::from_str::<UserSettings>(message_str) {
        return Some(MessageTypes::SetUserSettings(msg));
    }

    if let Ok(msg) = serde_json::from_str::<Prompt>(message_str) {
        return Some(MessageTypes::GetTextCompletion(msg));
    }

    if let Ok(msg) = serde_json::from_str::<UpdateAction>(message_str) {
        return Some(MessageTypes::UpdateAction(msg));
    }

    println!("Could not parse message: {}", message_str);

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

// Your existing type definitions here...

async fn get_openai_completion(messages: Vec<ChatMessage>, api_key: String) -> Result<String> {
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
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );

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
        let chat_completion: ChatCompletion =
            serde_json::from_str(&response.text().await.unwrap())?;

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

async fn start_websocket_server(
    rx: Arc<tokio::sync::Mutex<UnboundedReceiver<(Identity, Message)>>>,
    client_tx: mpsc::Sender<(Identity, String)>,
) {
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();

    let request_dispatcher: HashMap<Identity, UnboundedSender<Message>> = HashMap::new();
    let thread_safe_request_dispatcher = Arc::new(Mutex::new(request_dispatcher));
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
            match sending_channel.send(outgoing_msg.1) {
                Ok(_res) => println!("sent message to client"),
                Err(_) => todo!(),
            }
        }
    });

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
                .insert(Identity::new(id.to_string()), local_tx);

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
async fn return_db(db_uri: String) -> mongodb::Database {
    let client_options = ClientOptions::parse(db_uri).await;

    match client_options {
        Ok(mut client_options) => {
            // Set the server_api field of the client_options object to Stable API version 1
            let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
            client_options.server_api = Some(server_api);
            // Get a handle to the cluster
            let client = Client::with_options(client_options).unwrap();

            client.database("skynet")
        }
        Err(e) => panic!("Error connecting to MongoDB: {:?}", e),
    }
}

// type DockerId = String;

struct RuntimeSettings {
    openai_api_key: String,
    mongo_db_uri: String,
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

    // let mut docker_containers: Vec<(Identity, DockerId)> = Vec::new();

    let mut runtime_settings: HashMap<Identity, RuntimeSettings> = HashMap::new();

    // get the database

    //read messages from the client
    while let Some(msg) = client_rx.recv().await {
        println!("Received a message from the client: {}", msg.1);

        let received_message: Option<MessageTypes> = parse_message(&msg.1);

        let message_contents: MessageTypes;

        if received_message.is_none() {
            print!("Received an invalid message from the client: {}", msg.1);
            continue;
        } else {
            message_contents = received_message.unwrap();
            println!(
                "Received a parsed message from the client: {:?}",
                message_contents
            );
        }

        match message_contents {
            MessageTypes::Goal(_) => todo!(),
            MessageTypes::InitializeProject(_) => {
                // get the actions and processes from the db

                // send the actions to the client

                let db_uri = runtime_settings.get(&msg.0).unwrap().mongo_db_uri.clone();

                let db = return_db(db_uri).await;

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
            MessageTypes::SetUserSettings(settings) => {
                println!("Setting openai key for {}", msg.0.name);

                // attempt to set them from environment variables
                let user_settings = UserSettings::new();

                if user_settings.is_some() {
                    let user_settings = user_settings.unwrap();
                    runtime_settings.insert(
                        msg.0.clone(),
                        RuntimeSettings {
                            openai_api_key: user_settings.openai_api_key,
                            mongo_db_uri: user_settings.mongo_db_uri,
                        },
                    );
                } else {
                    runtime_settings.insert(
                        msg.0.clone(),
                        RuntimeSettings {
                            openai_api_key: settings.openai_api_key,
                            mongo_db_uri: settings.mongo_db_uri,
                        },
                    );
                }

                // respond to the client
                match tx.send((
                    Identity::new(msg.0.name.to_string()),
                    Message::Text("Settings received".to_string()),
                )) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error sending message to client: {:?}", e);
                        break;
                    }
                }
            }
            MessageTypes::GetTextCompletion(prompt) => {
                // check to see if the client has an openai key
                let openai_api_key = match runtime_settings.get(&msg.0) {
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
                        content: prompt.prompt_text.clone()
                    }
                );

                    let response = get_openai_completion(messages, openai_api_key.unwrap()).await;

                    match response {
                        Ok(res) => {
                            match tx
                                .send((Identity::new(msg.0.name.to_string()), Message::Text(res)))
                            {
                                Ok(_) => {}
                                Err(e) => {
                                    println!("Error sending message to client: {:?}", e);
                                    break;
                                }
                            }
                        }
                        Err(_) => todo!(),
                    }
                }

                println!("Received text completion from {}", msg.0.name);
            }
            MessageTypes::UpdateAction(update_action) => {
                let updated_action = update_action.action;

                let db_uri = runtime_settings.get(&msg.0).unwrap().mongo_db_uri.clone();

                let db = return_db(db_uri).await;

                let action_collection = db.collection::<Action>("actions");

                let filter = doc! { "_id": updated_action._id.clone().unwrap() };

                let update = doc! { "$set": { "name": updated_action.name.clone(), "prompt":

                    updated_action.prompt.clone(),  "system" : updated_action.system.clone(), "input_variables" : updated_action.input_variables.clone(), "output_variables": updated_action.output_variables.clone() }
                };

                let update_result = action_collection
                    .update_one(filter, update, None)
                    .await
                    .unwrap();

                if update_result.modified_count == 0 {
                    println!("No actions updated");
                } else {
                    println!("Updated {} actions", update_result.modified_count);

                    match tx.send((
                        Identity::new(msg.0.name.to_string()),
                        Message::Text(json!(updated_action).to_string()),
                    )) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Error sending message to client: {:?}", e);
                            break;
                        }
                    }
                }
            }
            MessageTypes::CreateAction(create_action) => {
                let db_uri = runtime_settings.get(&msg.0).unwrap().mongo_db_uri.clone();

                let db = return_db(db_uri).await;

                let action_collection = db.collection::<Action>("actions");

                let mut action = create_action.create_action.clone();

                action._id = Some(bson::oid::ObjectId::new());

                let insert_result = action_collection.insert_one(action, None).await.unwrap();

                println!("Inserted action: {}", insert_result.inserted_id);

                let inserted_action = action_collection
                    .find_one(doc! { "_id": insert_result.inserted_id.clone() }, None)
                    .await
                    .unwrap()
                    .unwrap();

                // send the created action back to the client
                let created_action: Action = inserted_action;

                let response = CreateAction {
                    create_action: created_action,
                };

                match tx.send((
                    Identity::new(msg.0.name.to_string()),
                    Message::Text(json!(response).to_string()),
                )) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error sending message to client: {:?}", e);
                        break;
                    }
                }
            }
            MessageTypes::CreateProcess(create_process) => {
                let db_uri = runtime_settings.get(&msg.0).unwrap().mongo_db_uri.clone();

                let db = return_db(db_uri).await;

                let process_collection = db.collection::<Process>("processes");

                let mut process = create_process.create_process.clone();

                process._id = Some(bson::oid::ObjectId::new());

                let insert_result = process_collection.insert_one(process, None).await.unwrap();

                println!("Inserted process: {}", insert_result.inserted_id);

                let inserted_process = process_collection
                    .find_one(doc! { "_id": insert_result.inserted_id.clone() }, None)
                    .await
                    .unwrap()
                    .unwrap();

                // send the created process back to the client
                let created_process: Process = inserted_process;

                let response = CreateProcess {
                    create_process: created_process,
                };

                match tx.send((
                    Identity::new(msg.0.name.to_string()),
                    Message::Text(json!(response).to_string()),
                )) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error sending message to client: {:?}", e);
                        break;
                    }
                }
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
        if let Ok(process) = process {
            processes.push(process);
        }
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
