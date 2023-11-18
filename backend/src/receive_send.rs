use crate::generated_types::{self, AuthenticationMessage, Identity, Node, NodeTypes, Prompt};
use crate::generated_types::{
    body::Contents, node_content::NodeContent as NodeContentEnum, Body, Edge, Envelope, Graph,
    GraphNodeInfo, Letter, Process, UserSettings, VerbTypes,
};

use async_openai::types::{ChatCompletionRequestUserMessage, ChatCompletionResponseFormat};
use async_openai::{
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestUserMessageContent,
        ChatCompletionResponseFormatType, CreateChatCompletionRequest, Role,
    },
    Client,
};
use handlebars::Handlebars;
// use std::env;

// use crate::graph::validate_nodes_from_process;

use colored::*;
// use petgraph::prelude::DiGraph;
// use petgraph::Direction;

use std::sync::Arc;

// use crate::utils::parse_message;
use crate::graph::validate_nodes_in_process;
use crate::sqlite_helper_functions::{fetch_all_nodes, insert_node, update_node};

use crate::SERVER_IDENTITY;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::sync::mpsc::UnboundedSender;

use prost::Message;

use prost::bytes::BytesMut;

// use petgraph::prelude::Bfs;
// use petgraph::algo::toposort;

// use bollard::container::Config;
// use bollard::exec::{ CreateExecOptions, StartExecResults };
// use bollard::Docker;
use bson::doc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tokio::sync::mpsc;
// use tokio_tungstenite::tungstenite::Message;

// create a "models" type that can be used to select the model to use
// it should be one of a couple of strings: "gpt-4", "gpt3.5-turbo", etc
// const DEFAULT_MODEL: &str = "gpt-4";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LocalServerIdentity {
    pub name: String,
}

impl LocalServerIdentity {
    pub fn new(name: String) -> LocalServerIdentity {
        LocalServerIdentity { name }
    }
}

pub async fn start_message_sending_loop(
    // docker: Docker,
    tx: UnboundedSender<(LocalServerIdentity, tokio_tungstenite::tungstenite::Message)>,
    mut client_rx: mpsc::Receiver<(LocalServerIdentity, tokio_tungstenite::tungstenite::Message)>,
    pool: Arc<Pool<SqliteConnectionManager>>,
) {
    let runtime_settings: HashMap<LocalServerIdentity, UserSettings> = HashMap::new();

    while let Some(msg) = client_rx.recv().await {
        println!(
            "{} {:?}",
            "Received a message from the client:".yellow(),
            msg.1.len()
        );

        // let received_message: Option<CrudBundle> = parse_message(&msg.1);

        // println!("message data: {:?}",msg.1.into_data());

        let slice = msg.1.clone().into_data().as_slice().to_vec();
        let envelope: Envelope;

        match Envelope::decode(&*slice) {
            Ok(val) => {
                envelope = val;
            }
            Err(err) => {
                println!("Error decoding message: {:?}", err);
                continue;
            }
        }

        // This is a special case where there is no receiver specified and therefore the message content can be ignored. It is assumed that the client is requesting the server identity
        if envelope.receiver.is_none() {
            println!("{}", "No receiver specified.".red());

            if envelope.sender.is_some() {
                println!("{}", "Sending server identity to client".green());

                let server_identity = SERVER_IDENTITY.get().unwrap();

                let body = Body {
                    contents: Some(Contents::Identity(server_identity.clone())),
                };

                let message_content = Letter {
                    verb: VerbTypes::Acknowledge as i32,
                    body: Some(body),
                };

                let vectorized_message = vec![message_content];

                let return_envelope = Envelope {
                    sender: Some(server_identity.clone()),
                    receiver: envelope.clone().sender.clone(),
                    letters: vectorized_message,
                    verification_id: envelope.verification_id.clone(),
                };

                send_message(&tx, msg.0.clone(), return_envelope).await;
            } else {
                println!("{}", "No sender, can't send message".red());
            }

            continue;
        }

        // check to see if the envelope receiver is NOT the same as the server identity in which case the server will attempt to send the message to the correct receiver
        if envelope.clone().receiver.unwrap() != *SERVER_IDENTITY.get().unwrap() {
            println!("{}", "Forward the message to the correct receiver".red());
        }

        println!(
            "{}",
            "TODO: Collection responses and send them in envelope batch.".red()
        );

        // loop through the letters and handle each one
        for letter in envelope.clone().letters {
            println!("Message content: {:?}", letter);
            let verb: VerbTypes = VerbTypes::try_from(letter.verb).unwrap();
            let sender: Identity = envelope.clone().sender.unwrap();
            let receiver: generated_types::Identity = envelope.clone().receiver.unwrap();
            let wrapped_content = letter.body.clone();
            let verification_id = envelope.clone().verification_id;

            let content: Contents;

            match wrapped_content {
                None => {
                    println!("{} {:?}", "No contents found:".red(), letter);
                    continue; // We should probably log this later... But we don't want to interrupt the message processing loop
                }
                Some(body) => {
                    content = body.contents.unwrap();
                }
            }

            match content {
                Contents::Node(node) => {
                    match verb {
                        VerbTypes::Create => {
                            let mut mutable_node = node.clone();

                            println!("Creating node: {:?}", mutable_node);

                            let new_node_info = GraphNodeInfo {
                                id: uuid::Uuid::new_v4().to_string(),
                                description: node.clone().node_info.unwrap().description.clone(),
                                name: node.node_info.clone().unwrap().name.clone(),
                            };

                            mutable_node.node_info = Some(new_node_info.clone());

                            let body = Body {
                                contents: Some(Contents::Node(mutable_node.clone())),
                            };

                            let letter = generated_types::Letter {
                                verb: VerbTypes::Acknowledge as i32,
                                body: Some(body),
                            };

                            //insert the node into the db
                            match insert_node(pool.clone(), mutable_node.clone()) {
                                Ok(_) => {
                                    println!("Node inserted successfully");

                                    let response_object = Envelope {
                                        sender: Some(receiver.clone()),
                                        receiver: Some(sender.clone()),
                                        letters: vec![letter],
                                        verification_id: verification_id.clone(),
                                    };

                                    send_message(&tx, msg.0.clone(), response_object).await;
                                }
                                Err(err) => {
                                    println!("Error inserting node: {:?}", err);
                                }
                            }
                        }
                        VerbTypes::Update => {
                            let updated_node = node.clone();

                            let body = Body {
                                contents: Some(Contents::Node(updated_node.clone())),
                            };

                            let letter = generated_types::Letter {
                                verb: VerbTypes::Acknowledge as i32,
                                body: Some(body),
                            };

                            match update_node(pool.clone(), &updated_node) {
                                Ok(_) => {
                                    println!("Node updated successfully");

                                    let updated_envelope = Envelope {
                                        sender: Some(receiver.clone()),
                                        receiver: Some(sender.clone()),
                                        letters: vec![letter],
                                        verification_id: verification_id.clone(),
                                    };

                                    send_message(&tx, msg.0.clone(), updated_envelope).await;
                                }
                                Err(err) => {
                                    println!("Error updating node: {:?}", err);
                                }
                            }
                        }

                        VerbTypes::Get => {
                            println!("{}", "TODO: Handle authentication message validation".red());

                            // Code to handle the initiation of authentication

                            let mut response_envelope: Envelope = Envelope {
                                sender: Some(receiver.clone()),
                                receiver: Some(sender.clone()),
                                letters: Vec::new(),
                                verification_id: verification_id.clone(),
                            };

                            match fetch_all_nodes(pool.clone()) {
                                Ok(nodes) => {
                                    for node in &nodes {
                                        println!("Found node: {:?}", node);

                                        let body = Body {
                                            contents: Some(Contents::Node(node.clone())),
                                        };

                                        let contents: generated_types::Letter =
                                            generated_types::Letter {
                                                verb: VerbTypes::Acknowledge as i32,
                                                body: Some(body),
                                            };

                                        response_envelope.letters.push(contents);
                                    }

                                    send_message(&tx, msg.0.clone(), response_envelope).await;
                                }
                                Err(err) => {
                                    println!(
                                        "Have the following errors when attempting to pull nodes from sqlite : {:?}",
                                        err
                                    );
                                }
                            }
                        }
                        VerbTypes::Execute => todo!(),
                        _ => {
                            println!("{} {:?}", "Verb not supported for node:".red(), verb);
                        }
                    }
                }
                Contents::AuthenticationMessage(_auth) => {
                    match verb {
                        VerbTypes::Initiate => {
                            // Closing the match fetch_all_nodes
                        } // Closing the VerbTypes::Initiate match arm
                        _ => {
                            println!("{}", "Authentication message not *yet* supported:".red());
                        }
                    } // Closing the match verb
                }
                Contents::UserSettings(user_settings) => {}
                Contents::NodesToProcess(nodes_to_process) => {
                    match verb {
                        VerbTypes::Validate => {
                            let outer_node_info = nodes_to_process.containing_node_info.clone();

                            let nodes = nodes_to_process.nodes.clone();
                            match validate_nodes_in_process(nodes, outer_node_info.unwrap()) {
                                Ok(mutable_node) => {
                                    println!("Nodes validated successfully");

                                    match insert_node(pool.clone(), mutable_node.clone()) {
                                        Ok(_) => {
                                            println!("Node inserted successfully");

                                            // we construct a new letter with the new mutable_node:

                                            let body = Body {
                                                contents: Some(Contents::Node(
                                                    mutable_node.clone(),
                                                )),
                                            };

                                            let letter = generated_types::Letter {
                                                verb: VerbTypes::Acknowledge as i32,
                                                body: Some(body),
                                            };

                                            let response_object = Envelope {
                                                sender: Some(receiver.clone()),
                                                receiver: Some(sender.clone()),
                                                letters: vec![letter],
                                                verification_id: verification_id.clone(),
                                            };

                                            send_message(&tx, msg.0.clone(), response_object).await;
                                        }
                                        Err(err) => {
                                            println!("Error inserting node: {:?}", err);
                                        }
                                    }

                                    // Add process node to the database

                                    // Send process back to the frontend
                                }
                                Err(err) => {
                                    println!("Error validating nodes: {:?}", err);
                                }
                            }
                        }
                        _ => {
                            println!(
                                "{} {:?}",
                                "Verb not supported for node:".red(),
                                letter.clone()
                            );
                        }
                    }
                }
                Contents::ExecutionDetails(execution) => {
                    match verb {
                        VerbTypes::Execute => {
                            // Keep track of the variable definitions (accumulate their values as we loop through the topological order list)

                            let mut variable_definitions: HashMap<String, String> =
                                execution.clone().current_variable_definitions;

                            let local_nodes: Vec<Node> =
                                execution.process.clone().unwrap().nodes.clone();

                            // Make a map out of the vec where the key is the id of the node:
                            let mut local_nodes_map: HashMap<String, Node> = HashMap::new();
                            local_nodes.iter().for_each(|node: &Node| {
                                local_nodes_map
                                    .insert(node.node_info.clone().unwrap().id, node.clone());
                            });

                            let topological_order: Vec<GraphNodeInfo> =
                                execution.process.clone().unwrap().topological_order.clone();

                            // Loop through the topological order list and execute each node in order

                            let mut node_execution_response: HashMap<String, String> =
                                HashMap::new();

                            for node_info in topological_order {
                                let current_node =
                                    local_nodes_map.get(&node_info.id).unwrap().clone();

                                // Process the node, which ever type it is:

                                // let process = NodeTypes::Process as i32;
                                // let prompt = NodeTypes::Prompt as i32;

                                match NodeTypes::try_from(current_node.node_type) {
                                    Ok(NodeTypes::Process) => {
                                        // Once we implement this functionality just for Prompts (and other node types), we can extract this function and call it recursively to handle this case (with a max depth?)
                                    }
                                    Ok(NodeTypes::Prompt) => {
                                        // we need to replace the prompt text input_variables with their definitions

                                        let prompt_text: String;

                                        let  additional_instruction =
                                            "When coming up with a response, please make the fields of the json response be the following: ".to_string();

                                        // Concatenate the strings in the vector to make a comma separated string.

                                        let variable_string: String =
                                            current_node.output_variables.join(", ");

                                        match current_node
                                            .node_content
                                            .unwrap()
                                            .node_content
                                            .unwrap()
                                        {
                                            NodeContentEnum::Prompt(prompt) => {
                                                // hydrate the prompt text with the variable definitions

                                                let mut handlebars = Handlebars::new();

                                                let json_variable_definitions: Value =
                                                    serde_json::json!(variable_definitions);

                                                handlebars
                                                    .register_template_string(
                                                        "prompt",
                                                        prompt.clone().prompt,
                                                    )
                                                    .unwrap();

                                                let hydrated_prompt = handlebars
                                                    .render("prompt", &json_variable_definitions)
                                                    .unwrap();

                                                prompt_text = format!(
                                                    "{} {} {}",
                                                    hydrated_prompt.clone(),
                                                    additional_instruction,
                                                    variable_string
                                                );

                                                print!("Prompt text: {}", prompt_text.green())
                                            }
                                            _ => {
                                                println!("prompt not handled");
                                                continue;
                                            }
                                        }

                                        // For now use the system api key, in the future this will be gathered from the user settings

                                        // let api_key = env::var("OPENAI_API_KEY").unwrap();

                                        // Create client

                                        // I believe this already pulls the key from the environmental variable.
                                        let client = Client::new();

                                        // .config().with_api_key(api_key);

                                        // Create the message from the prompt:

                                        let user_message = ChatCompletionRequestUserMessage {
                                            content: Some(
                                                ChatCompletionRequestUserMessageContent::Text(
                                                    prompt_text,
                                                ),
                                            ),
                                            role: Role::User,
                                        };

                                        let message: ChatCompletionRequestMessage =
                                            ChatCompletionRequestMessage::User(user_message);

                                        // Create request using builder pattern
                                        // Every request struct has companion builder struct with same name + Args suffix
                                        let mut request = CreateChatCompletionRequest::default();

                                        request.messages = vec![message];
                                        request.response_format =
                                            Some(ChatCompletionResponseFormat {
                                                r#type:
                                                    ChatCompletionResponseFormatType::JsonObject,
                                            });
                                        request.model = "gpt-3.5-turbo-instruct".to_string();

                                        // Call API
                                        let response = client.chat().create(request).await.unwrap();

                                        println!(
                                            "{}",
                                            response
                                                .choices
                                                .first()
                                                .unwrap()
                                                .message
                                                .content
                                                .clone()
                                                .unwrap()
                                                .as_str()
                                                .to_string()
                                        );

                                        let json_string = response
                                            .choices
                                            .first()
                                            .unwrap()
                                            .message
                                            .content
                                            .clone()
                                            .unwrap()
                                            .as_str()
                                            .to_string();

                                        node_execution_response
                                            .insert(node_info.clone().id, json_string.clone());

                                        let value: Value =
                                            serde_json::from_str(json_string.as_str()).unwrap();
                                        if let Some(obj) = value.as_object() {
                                            for (key, value) in obj {
                                                println!("{}: {}", key, value);
                                                variable_definitions
                                                    .insert(key.clone(), value.clone().to_string());
                                            }
                                        } else {
                                            println!("{}", "The JSON is not an object.".red());
                                        }

                                        // see if all of the output_variables of the node are in the variable definition hashmap:

                                        let check_output_vars: Vec<String> =
                                            current_node.output_variables;

                                        //loop through check_output_vars and see if the key exists in the variable_definitions hashmap:

                                        for output_var in check_output_vars.iter() {
                                            if !variable_definitions.contains_key(output_var) {
                                                println!("Missing variable: {}", output_var.red());
                                                // Handle the missing variable case here (e.g., error handling or logging)

                                                // Send back the Execution to the frontend and let the user decide what to do.

                                                let mut error_response = execution.clone();

                                                error_response.current_node =
                                                    Some(node_info.clone());
                                                error_response.current_variable_definitions =
                                                    variable_definitions.clone();

                                                let letter = Letter {
                                                    body: Some(Body {
                                                        contents: Some(Contents::ExecutionDetails(
                                                            error_response,
                                                        )),
                                                    }),

                                                    verb: VerbTypes::Error as i32,
                                                };

                                                let envelope = Envelope {
                                                    letters: vec![letter],
                                                    sender: Some(receiver.clone()),
                                                    receiver: Some(sender.clone()),
                                                    verification_id: verification_id.clone(),
                                                };

                                                send_message(&tx, msg.0.clone(), envelope).await;
                                            }
                                        }
                                    }

                                    _ => {
                                        println!("Other types not implemented yet");
                                        continue;
                                    }
                                }
                            }

                            let mut response = execution.clone();
                            response.node_execution_response = node_execution_response;
                            response.current_variable_definitions = variable_definitions.clone();

                            let letter = Letter {
                                body: Some(Body {
                                    contents: Some(Contents::ExecutionDetails(response)),
                                }),

                                verb: VerbTypes::Acknowledge as i32,
                            };

                            let envelope = Envelope {
                                letters: vec![letter],
                                sender: Some(receiver.clone()),
                                receiver: Some(sender.clone()),
                                verification_id: verification_id.clone(),
                            };

                            send_message(&tx, msg.0.clone(), envelope).await;
                        }
                        _ => {
                            println!(
                                "{} {:?}",
                                "Execution details not *yet* supported for this verb:".red(),
                                verb.clone()
                            );
                        }
                    }
                }
                _ => {
                    println!("{}", "Not yet implemented".red());
                }
            }

            // match message_content.contents {
            //     Some(Contents::CrudBundle(crud_bundle)) => {
            //         handle_crud_bundle(
            //             tx.clone(),
            //             msg.clone(),
            //             crud_bundle,
            //             pool.clone(),
            //             &mut runtime_settings
            //         ).await;
            //     }
            //     Some(Contents::Identity(identity)) => {
            //         println!("Identity: {:?}", identity);
            //     }
            //     Some(Contents::UserSettings(user_settings)) => {
            //         println!("User Settings: {:?}", user_settings);
            //     }
            //     Some(Contents::ExecutionContext(execution_context)) => {
            //         println!("Execution Context: {:?}", execution_context);
            //     }
            //     Some(Contents::ValidateNodesResponse(validate_nodes_response)) => {
            //         println!("Validate Nodes Response: {:?}", validate_nodes_response);
            //     }
            //     None => {
            //         println!("No contents found");
            //     }
            // }
        }
    }

    // match message_contents.object {
    //     Some(crud_bundle::Object::Node(node)) => {
    //         match verb {
    //             VerbTypeNames::Post => {
    //
    //             }
    //             VerbTypeNames::Put => {

    //             }
    //             _ => {
    //                 println!("Verb not supported for node: {:?}", verb);
    //             }
    //         }
    // }
    // Some(crud_bundle::Object::AuthenticationMessage(_authentication_message)) => {
    //     match verb {
    //         VerbTypeNames::Post => {
    //             println!("Initializing project for {}", msg.0.name);
    //             println!(
    //                 "Found the following settings: {:?}",
    //                 runtime_settings.get(&msg.0)
    //             );

    //             println!("Get nodes, settings, etc from db!");

    //             match fetch_all_nodes(pool.clone()) {
    //                 Ok(nodes) => {
    //                     for node in &nodes {
    //                         println!("Found node: {:?}", node);

    //                         send_message(&tx, msg.0.clone(), ResponseObject {
    //                             object: Some(Node(node.clone())),
    //                         }).await;
    //                     }
    //                 }
    //                 Err(err) => {
    //                     println!(
    //                         "Have the following errors when attempting to pull nodes from sqlite : {:?}",
    //                         err
    //                     );
    //                 }
    //             }
    //         }
    //         _ => {
    //             println!("Verb not supported for initial message: {:?}", verb);
    //         }
    //     }
    // }
    // Some(crud_bundle::Object::UserSettings(_user_settings)) => {
    //     match verb {
    //         VerbTypeNames::Get => {
    //             println!("Setting user settings for {}", msg.0.name);

    //             // attempt to set them from environment variables
    //             let system_settings = UserSettings::new();

    //             match system_settings {
    //                 Some(settings) => {
    //                     println!("settings: {:?}", settings);

    //                     // Check if runtime_settings already have settings for the user
    //                     if runtime_settings.contains_key(&msg.0) {
    //                         println!("Settings for user {} already exist", msg.0.name);
    //                     } else {
    //                         runtime_settings.insert(msg.0.clone(), UserSettings {
    //                             openai_api_key: settings.openai_api_key,
    //                             mongo_db_uri: settings.mongo_db_uri,
    //                         });
    //                         println!("Settings for user {} have been set", msg.0.name);
    //                     }
    //                 }
    //                 None => {
    //                     // runtime_settings.insert(msg.0.clone(), UserSettings {
    //                     //     openai_api_key: user_settings.openai_api_key,
    //                     //     mongo_db_uri: user_settings.mongo_db_uri,
    //                     // });
    //                     panic!("fug... the settings are not set.");
    //                 }
    //             }

    //             // respond to the client
    //             // send_message(&tx, msg.0.clone(), ResponseObject::UserSettings).await;

    //             todo!("send some acknowledgement that user settings are in the system");
    //         }
    //         _ => {
    //             println!(
    //                 "\n-------------------\nVerb not supported for user settings: {:?}\n-------------------\n",
    //                 verb
    //             );
    //         }
    //     }
    // }
    // Some(crud_bundle::Object::ExecutionContext(_execution_context)) => {
    //     match verb {
    //         _ => {
    //             todo!("Handle execution context");
    //         }
    //     }
    // }
    // Some(crud_bundle::Object::ValidateNodes(node_container)) => {
    //     match verb {
    //         VerbTypeNames::Post => {
    //
    // }

    // None => {
    //     println!("odd...");
    //     println!(
    //         "This probably means that the websocket connection has closed... Should remove it from the identity hash"
    //     );
    // }
}
// }

pub async fn send_message(
    tx: &UnboundedSender<(LocalServerIdentity, tokio_tungstenite::tungstenite::Message)>,
    identity: LocalServerIdentity,
    envelope: Envelope,
) {
    let mut buf = BytesMut::new();
    envelope.encode(&mut buf).unwrap();

    println!("{}: {:?}", "Sending message to client".green(), envelope);

    match tx.send((
        identity,
        tokio_tungstenite::tungstenite::Message::Binary(buf.to_vec()),
    )) {
        Ok(_) => {}
        Err(e) => {
            println!("Error sending message to client: {:?}", e);
        }
    }
}
