use crate::domain::{
    CrudBundle,
    Node,
    NodeType,
    Response,
    CrudBundleObject,
    VerbTypeNames,
    ResponseObject,
    ExecutionResponse,
    CommandResponse,
    ExecutionContext,
};
use crate::mongo::{ get_nodes, return_db };
use crate::openai::{ get_openai_completion, ChatMessage, Role };
use crate::settings::{ RuntimeSettings, UserSettings };
use crate::utils::{ parse_message, create_node_response_object };

use bollard::container::Config;
use bollard::exec::{ CreateExecOptions, StartExecResults };
use bollard::Docker;
use bson::doc;
use bson::Bson;
use bson::Document;
use futures_util::StreamExt;
use serde::{ Deserialize, Serialize };
use serde_json::json;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;

// create a "models" type that can be used to select the model to use
// it should be one of a couple of strings: "gpt-4", "gpt3.5-turbo", etc
const default_model = "gpt-4";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Identity {
    pub name: String,
}

impl Identity {
    pub fn new(name: String) -> Identity {
        Identity { name }
    }
}

pub async fn start_message_sending_loop(
    // docker: Docker,
    tx: UnboundedSender<(Identity, Message)>,
    mut client_rx: mpsc::Receiver<(Identity, String)>
) {
    let mut runtime_settings: HashMap<Identity, UserSettings> = HashMap::new();
    let mut messages_thus_far: HashMap<Identity, Vec<String>> = HashMap::new();
    let mut docker_containers: HashMap<Identity, String> = HashMap::new();

    // startup the docker container here
    let docker = Docker::connect_with_local_defaults().unwrap();
    //read messages from the client
    while let Some(msg) = client_rx.recv().await {
        println!("Received a message from the client: {}", msg.1);

        let received_message: Option<CrudBundle> = parse_message(&msg.1);

        let message_contents: CrudBundle;

        if received_message.is_none() {
            print!("Received an invalid message from the client: {}", msg.1);
            continue;
        } else {
            message_contents = received_message.unwrap();
            println!("Received a parsed message from the client: {:?}", message_contents);
        }

        let verb: VerbTypeNames = message_contents.verb.clone();

        match message_contents.object {
            CrudBundleObject::Node(node) => {
                match verb {
                    VerbTypeNames::POST => {
                        let db_uri = runtime_settings.get(&msg.0).unwrap().mongo_db_uri.clone();

                        let db = return_db(db_uri).await;

                        let node_collection = db.collection::<crate::domain::Node>("nodes");

                        let mut node = node.node_content.clone();

                        node._id = Some(bson::oid::ObjectId::new());

                        let insert_result = node_collection.insert_one(node, None).await.unwrap();

                        println!("Inserted node: {:?}", insert_result);

                        let inserted_node = node_collection
                            .find_one(doc! { "id": insert_result.inserted_id.clone() }, None).await
                            .unwrap()
                            .unwrap();

                        let response_object = ResponseObject::Node(inserted_node);

                        send_message(&tx, msg.0.clone(), response_object);
                    }
                    VerbTypeNames::PUT => {
                        let updated_node = update_node.node.clone();

                        let db_uri = runtime_settings.get(&msg.0).unwrap().mongo_db_uri.clone();

                        let db = return_db(db_uri).await;

                        let node_collection = db.collection::<Node>("nodes");

                        let filter = doc! { "_id": updated_node._id.clone().unwrap() };

                        let update = match updated_node.node_content.clone() {
                            NodeType::Prompt(prompt) => {
                                doc! {
                            "$set": {
                                "Prompt": {
                                "prompt": prompt.prompt.clone(),
                                "system": prompt.system.clone(),
                                "input_variables": prompt.input_variables.clone(),
                                "output_variables": prompt.output_variables.clone()
                                }
                            }
                        }
                            }
                            NodeType::Process(process) => {
                                doc! {
                            "$set": {
                                "Process": {
                                "graph": process.graph.clone(),
                                "topological_order": process.topological_order.clone(),
                                "description": process.description.clone(),
                                "output_variable": process.output_variable.clone(),
                                "is_loop": process.is_loop,
                                "max_iterations": process.max_iterations.clone()
                                }
                            }
                        }
                            }
                            NodeType::Conditional(conditional) => {
                                let mut system_variables = doc! {};

                                for (key, value) in conditional.system_variables {
                                    system_variables.insert(key, value);
                                }

                                let mut new_options = Document::new();
                                for (key, value) in &conditional.options {
                                    new_options.insert(key.clone(), Bson::from(value.clone()));
                                }

                                doc! {
                            "$set": {
                                "Conditional": {
                                "system_variables": system_variables,
                                "statement": conditional.statement.clone(),
                                "options": new_options
                                }
                            }
                        }
                            }
                            NodeType::Command(command) => {
                                doc! {
                            "$set": {
                                "Command": {
                                "command": command.command.clone()
                                }
                            }
                        }
                            }
                        };

                        let update_result = node_collection
                            .update_one(filter, update, None).await
                            .unwrap();

                        if update_result.modified_count == 0 {
                            println!("No nodes updated");
                        } else {
                            println!("Updated {} nodes", update_result.modified_count);

                            send_message(&tx, msg.0.clone(), updated_node);
                        }
                    }
                    _ => {
                        println!("Verb not supported for node: {:?}", verb);
                    }
                }
            }
            CrudBundleObject::InitialMessage(initial_message) => {
                match verb {
                    VerbTypeNames::POST => {
                        println!("Initializing project for {}", msg.0.name);
                        println!(
                            "Found the following settings: {:?}",
                            runtime_settings.get(&msg.0)
                        );

                        let db_uri = runtime_settings.get(&msg.0).unwrap().mongo_db_uri.clone();

                        let db = return_db(db_uri).await;

                        let nodes = get_nodes(&db).await;

                        println!("Found the following nodes: {:?}", nodes);

                        for node in &nodes {
                            send_message(
                                &tx,
                                msg.0.clone(),
                                ResponseObject::Node(node.clone())
                            ).await;
                        }

                        // need to send an additional message to the client to let them know that the project has been initialized

                        const IMAGE: &str = "alpine:3";

                        let alpine_config = Config {
                            image: Some(IMAGE),
                            tty: Some(true),
                            attach_stdin: Some(true),
                            attach_stdout: Some(true),
                            attach_stderr: Some(true),
                            open_stdin: Some(true),
                            ..Default::default()
                        };

                        let id = docker
                            .create_container::<&str, &str>(None, alpine_config.clone()).await
                            .unwrap().id;

                        println!("Created container with id: {}", id);
                        docker_containers.insert(msg.0.clone(), id);

                        send_message(&tx, msg.0.clone(), ResponseObject::InitialMessage).await;
                    }
                    _ => {
                        println!("Verb not supported for initial message: {:?}", verb);
                    }
                }
            }
            CrudBundleObject::UserSettings(user_settings) => {
                match verb {
                    VerbTypeNames::GET => {
                        println!("Setting user settings for {}", msg.0.name);

                        // attempt to set them from environment variables
                        let system_settings = UserSettings::new();

                        if user_settings.is_some() {
                            let user_settings = user_settings.unwrap();
                            runtime_settings.insert(msg.0.clone(), RuntimeSettings {
                                openai_api_key: user_settings.openai_api_key,
                                mongo_db_uri: user_settings.mongo_db_uri,
                            });
                        } else {
                            let settings = system_settings.unwrap();
                            runtime_settings.insert(msg.0.clone(), RuntimeSettings {
                                openai_api_key: settings.openai_api_key,
                                mongo_db_uri: settings.mongo_db_uri,
                            });
                        }

                        let users_runtime_settings = runtime_settings.get(&msg.0).unwrap();

                        // respond to the client
                        send_message(
                            &tx,
                            msg.0.clone(),
                            ResponseObject::UserSettings(users_runtime_settings)
                        );
                    }
                    _ => {
                        println!("Verb not supported for user settings: {:?}", verb);
                    }
                }
            }
            CrudBundleObject::ExecutionContext(execution_context) => {
                let node = execution_context.current_node.clone();
                let execution_clone: ExecutionContext = execution_context.clone();

                match node.node_content {
                    NodeType::Prompt(prompt) => {
                        match verb {
                            VerbTypeNames::POST => {
                                let openai_api_key = match runtime_settings.get(&msg.0) {
                                    Some(settings) => Some(settings.openai_api_key.clone()),
                                    None => {
                                        println!("No openai key set for {}", msg.0.name);
                                        None
                                    }
                                };

                                if openai_api_key.is_some() {
                                    let messages = vec![
                                        ChatMessage {
                                            role: Role::System,
                                            content: prompt.system
                                                .unwrap_or("".to_string())
                                                .clone(),
                                        },
                                        ChatMessage {
                                            role: Role::User,
                                            content: prompt.prompt.clone(),
                                        }
                                    ];

                                    let response = get_openai_completion(
                                        messages,
                                        openai_api_key.unwrap(),
                                        default_model.to_string()
                                    ).await;

                                    match response {
                                        Ok(res) => {
                                            response_object = create_node_response_object(
                                                execution_clone,
                                                NodeExecutionResponse::Prompt(PromptResponse {
                                                    response: res,
                                                })
                                            );

                                            send_message(&tx, msg.0.clone(), response_object).await;
                                        }
                                        Err(_) => todo!(),
                                    }
                                }
                            }
                            _ => {
                                println!("Verb not supported for prompt: {:?}", verb);
                            }
                        }
                    }
                    NodeType::Process(_) => {
                        println!(
                            "Processes cannot be executed directly. Instead, the frontend should break the process into nodes and send a execution context to the backend."
                        );
                    }
                    NodeType::Conditional(_) => todo!("Conditional not implemented yet"),
                    NodeType::Command(command) => {
                        match verb {
                            VerbTypeNames::POST => {
                                if let Some(container_id) = docker_containers.get(&msg.0) {
                                    let exec_options = CreateExecOptions {
                                        attach_stdout: Some(true),
                                        cmd: Some(vec!["sh", "-c", &command.command]),
                                        ..Default::default()
                                    };

                                    let exec_created = docker
                                        .create_exec(container_id, exec_options).await
                                        .unwrap();

                                    // Start the exec instance
                                    let exec_started = docker
                                        .start_exec(&exec_created.id, None).await
                                        .unwrap();

                                    match exec_started {
                                        StartExecResults::Attached { mut output, .. } => {
                                            let mut full_output = String::new(); // used to accumulate the output

                                            while let Some(item) = output.next().await {
                                                match item {
                                                    Ok(log) => {
                                                        println!("{:?}", log);
                                                        let log_str = log.to_string();
                                                        full_output.push_str(&log_str);
                                                        full_output.push('\n'); // add a newline between each piece of output
                                                    }
                                                    Err(e) => eprintln!("Error: {:?}", e),
                                                }
                                            }

                                            // Once we've read all the output, send it to the client

                                            let node_execution_response =
                                                NodeExecutionResponse::Command(CommandResponse {
                                                    error: "".to_string(),
                                                    output: full_output,
                                                });

                                            let response_object: ResponseObject =
                                                create_node_response_object(
                                                    execution_clone,
                                                    node_execution_response
                                                );

                                            send_message(&tx, msg.0.clone(), response_object).await;
                                        }
                                        StartExecResults::Detached => {
                                            println!(
                                                "The exec instance completed execution and detached"
                                            );
                                        }
                                    }
                                } else {
                                    println!("No container found for this client.");
                                }
                            }
                            _ => {
                                println!("Verb not supported for command: {:?}", verb);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub async fn send_message<T: Serialize + Sized>(
    tx: &UnboundedSender<(Identity, Message)>,
    identity: Identity,
    message: ResponseObject
) {
    match tx.send((identity, Message::Text(json!(message).to_string()))) {
        Ok(_) => {}
        Err(e) => {
            println!("Error sending message to client: {:?}", e);
        }
    }
}
