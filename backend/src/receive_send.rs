use crate::generated_types::{
    CrudBundle,
    VerbTypeNames,
    ResponseObject,
    ExecutionContext,
    NodeExecutionResponse,
    PromptResponse,
    UserSettings,
    CommandResponse,
    node,
    crud_bundle,
    node_execution_response,
};

use crate::openai::{ get_openai_completion, ChatMessage, Role };
use crate::utils::{ parse_message, create_node_response_object };

// use bollard::container::Config;
// use bollard::exec::{ CreateExecOptions, StartExecResults };
use bollard::Docker;
use bson::doc;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;
use crate::generated_types::node_execution_response::Response;

// create a "models" type that can be used to select the model to use
// it should be one of a couple of strings: "gpt-4", "gpt3.5-turbo", etc
const DEFAULT_MODEL: &str = "gpt-4";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Identity {
    pub name: String,
}

impl Identity {
    pub fn new(name: String) -> Identity {
        Identity { name }
    }
}

// use std::env;

pub async fn start_message_sending_loop(
    // docker: Docker,
    tx: UnboundedSender<(Identity, Message)>,
    mut client_rx: mpsc::Receiver<(Identity, String)>
) {
    let mut runtime_settings: HashMap<Identity, UserSettings> = HashMap::new();
    // let mut messages_thus_far: HashMap<Identity, Vec<String>> = HashMap::new();
    // let mut docker_containers: HashMap<Identity, String> = HashMap::new();

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

        let verb: VerbTypeNames = VerbTypeNames::from_i32(message_contents.verb).unwrap();

        match message_contents.object {
            Some(crud_bundle::Object::Node(node)) => {
                todo!("Add create node function for sqlite here");

                match verb {
                    VerbTypeNames::Post => {
                        let mut mutable_node = node.clone();

                        // create a uuid for the node:
                        mutable_node.id = uuid::Uuid::new_v4().to_string();

                        // get_sqlite_db is a function that returns a connection to the sqlite db

                        let connection = sqlite_helper_function::get_sqlite_db().unwrap();

                        //insert the node into the db
                        match 
                        sqlite_helper_function::insert_node(&connection, &mutable_node){
                            Ok(_) => {
                                println!("Node inserted successfully");
                                let response_object = ResponseObject { object: Node(mutable_node) };

                                send_message(&tx, msg.0.clone(), response_object).await;
                            }
                            Err(err) => {
                                println!("Error inserting node: {:?}", err);
                            }
                        }


                    },
                    VerbTypeNames::Put => {
                        let updated_node = node.clone();

                        let connection = sqlite_helper_function::get_sqlite_db().unwrap();

                        update_node(&connection, &updated_node).unwrap();

                    
                        let response_object: ResponseObject = ResponseObject {
                        object: Node(updated_node),
                        };

                        send_message(&tx, msg.0.clone(), response_object).await;
                    }

            }
            Some(crud_bundle::Object::AuthenticationMessage(_authentication_message)) => {
                match verb {
                    VerbTypeNames::Post => {
                        println!("Initializing project for {}", msg.0.name);
                        println!(
                            "Found the following settings: {:?}",
                            runtime_settings.get(&msg.0)
                        );

                        todo!("Get nodes, settings, etc from db!");

                        // let db_uri = runtime_settings.get(&msg.0).unwrap().mongo_db_uri.clone();

                        // let db = return_db(db_uri).await;

                        // let nodes = get_nodes(&db).await;

                        // println!("Found the following nodes: {:?}", nodes);

                        // for node in &nodes {
                        //     send_message(&tx, msg.0.clone(), ResponseObject {
                        //         object: Node(node.clone()),
                        //     }).await;
                        // }

                        // Get the docker image from env variables:
                        // const IMAGE: &str = env::var("DOCKER_OPERATING_SYSTEM").unwrap();

                        // let alpine_config = Config {
                        //     image: Some(IMAGE),
                        //     tty: Some(true),
                        //     attach_stdin: Some(true),
                        //     attach_stdout: Some(true),
                        //     attach_stderr: Some(true),
                        //     open_stdin: Some(true),
                        //     ..Default::default()
                        // };

                        // let id = docker
                        //     .create_container::<&str, &str>(None, alpine_config.clone()).await
                        //     .unwrap().id;

                        // println!("Created container with id: {}", id);
                        // docker_containers.insert(msg.0.clone(), id);

                        // // need to send an additional message to the client to let them know that the project has been initialized

                        // // send_message(&tx, msg.0.clone(), ResponseObject::AuthorizationToken).await;
                        // todo!(
                        //     "send auth token to user that will be required to execute other commands"
                        // );
                    }
                    _ => {
                        println!("Verb not supported for initial message: {:?}", verb);
                    }
                }
            }
            Some(crud_bundle::Object::UserSettings(_user_settings)) => {
                match verb {
                    VerbTypeNames::Get => {
                        println!("Setting user settings for {}", msg.0.name);

                        // attempt to set them from environment variables
                        let system_settings = UserSettings::new();

                        match system_settings {
                            Some(settings) => {
                                println!("settings: {:?}", settings);

                                // Check if runtime_settings already have settings for the user
                                if runtime_settings.contains_key(&msg.0) {
                                    println!("Settings for user {} already exist", msg.0.name);
                                } else {
                                    runtime_settings.insert(msg.0.clone(), UserSettings {
                                        openai_api_key: settings.openai_api_key,
                                        mongo_db_uri: settings.mongo_db_uri,
                                    });
                                    println!("Settings for user {} have been set", msg.0.name);
                                }
                            }
                            None => {
                                // runtime_settings.insert(msg.0.clone(), UserSettings {
                                //     openai_api_key: user_settings.openai_api_key,
                                //     mongo_db_uri: user_settings.mongo_db_uri,
                                // });
                                panic!("fug... the settings are not set.");
                            }
                        }

                        // respond to the client
                        // send_message(&tx, msg.0.clone(), ResponseObject::UserSettings).await;

                        todo!("send some acknowledgement that user settings are in the system");
                    }
                    _ => {
                        println!(
                            "\n-------------------\nVerb not supported for user settings: {:?}\n-------------------\n",
                            verb
                        );
                    }
                }
            }
            Some(crud_bundle::Object::ExecutionContext(execution_context)) => {
                match verb {
                    _ => {
                        todo!("Handle execution context")
                    }
                }
            }
            // Some(crud_bundle::Object::ExecutionContext(execution_context)) => {
            //     let node = execution_context.current_node.clone();
            //     let execution_clone: ExecutionContext = execution_context.clone();

            //     match node.node_content {
            //         Some(node::NodeContent::Prompt(prompt)) => {
            //             match verb {
            //                 VerbTypeNames::Post => {
            //                     let openai_api_key = match runtime_settings.get(&msg.0) {
            //                         Some(settings) => Some(settings.openai_api_key.clone()),
            //                         None => {
            //                             println!("No openai key set for {}", msg.0.name);
            //                             None
            //                         }
            //                     };

            //                     if openai_api_key.is_some() {
            //                         let messages = vec![
            //                             ChatMessage {
            //                                 role: Role::System,
            //                                 content: prompt.system.clone(),
            //                             },
            //                             ChatMessage {
            //                                 role: Role::User,
            //                                 content: prompt.prompt.clone(),
            //                             }
            //                         ];

            //                         let response = get_openai_completion(
            //                             messages,
            //                             openai_api_key.unwrap(),
            //                             DEFAULT_MODEL.to_string()
            //                         ).await;

            //                         match response {
            //                             Ok(res) => {
            //                                 let response_object = create_node_response_object(
            //                                     execution_clone,
            //                                     NodeExecutionResponse {
            //                                         response: Some(
            //                                             Response::PromptResponse(PromptResponse {
            //                                                 ai_text_response: res,
            //                                             })
            //                                         ),
            //                                     }
            //                                 );

            //                                 send_message(&tx, msg.0.clone(), response_object).await;
            //                             }
            //                             Err(_) => todo!(),
            //                         }
            //                     }
            //                 }
            //                 _ => {
            //                     println!("Verb not supported for prompt: {:?}", verb);
            //                 }
            //             }
            //         }
            //         Some(node::NodeContent::Process(_process)) => {
            //             println!(
            //                 "Processes cannot be executed directly. Instead, the frontend should break the process into nodes and send a execution context to the backend."
            //             );
            //         }
            //         Some(node::NodeContent::Conditional(_conditional)) => {
            //             todo!("Conditional not implemented yet");
            //         }

            //         Some(node::NodeContent::Command(command)) => {
            //             match verb {
            //                 VerbTypeNames::Post => {
            //                     if let Some(container_id) = docker_containers.get(&msg.0) {
            //                         let exec_options = CreateExecOptions {
            //                             attach_stdout: Some(true),
            //                             cmd: Some(vec!["sh", "-c", &command.command]),
            //                             ..Default::default()
            //                         };

            //                         let exec_created = docker
            //                             .create_exec(container_id, exec_options).await
            //                             .unwrap();

            //                         // Start the exec instance
            //                         let exec_started = docker
            //                             .start_exec(&exec_created.id, None).await
            //                             .unwrap();

            //                         match exec_started {
            //                             StartExecResults::Attached { mut output, .. } => {
            //                                 let mut full_output = String::new(); // used to accumulate the output

            //                                 while let Some(item) = output.next().await {
            //                                     match item {
            //                                         Ok(log) => {
            //                                             println!("{:?}", log);
            //                                             let log_str = log.to_string();
            //                                             full_output.push_str(&log_str);
            //                                             full_output.push('\n'); // add a newline between each piece of output
            //                                         }
            //                                         Err(e) => eprintln!("Error: {:?}", e),
            //                                     }
            //                                 }

            //                                 // Once we've read all the output, send it to the client

            //                                 let node_execution_response = NodeExecutionResponse {
            //                                     response: Some(
            //                                         node_execution_response::Response::CommandResponse(
            //                                             generated_types::CommandResponse {
            //                                                 error: None,
            //                                                 output: Some(full_output),
            //                                             }
            //                                         )
            //                                     ),
            //                                 };

            //                                 let response_object: ResponseObject =
            //                                     create_node_response_object(
            //                                         execution_clone,
            //                                         node_execution_response
            //                                     );

            //                                 send_message(&tx, msg.0.clone(), response_object).await;
            //                             }
            //                             StartExecResults::Detached => {
            //                                 println!(
            //                                     "The exec instance completed execution and detached"
            //                                 );
            //                             }
            //                         }
            //                     } else {
            //                         println!("No container found for this client.");
            //                     }
            //                 }
            //                 _ => {
            //                     println!("Verb not supported for command: {:?}", verb);
            //                 }
            //             }
            //         }
            //     }
            // }
            None => {
                println!("odd...");
            }
        }
    }
}
use crate::utils::to_u8_vec;

pub async fn send_message(
    tx: &UnboundedSender<(Identity, Message)>,
    identity: Identity,
    message: ResponseObject
) {
    match to_u8_vec(&message) {
        Ok(u8_vec) => {
            //convert u8_vec to string
            let send_string = String::from_utf8(u8_vec).unwrap();

            match tx.send((identity, Message::Text(send_string))) {
                Ok(_) => {}
                Err(e) => {
                    println!("Error sending message to client: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("Error encoding message: {:?}", e);
        }
    }
}
