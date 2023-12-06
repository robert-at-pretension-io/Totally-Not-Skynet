use crate::generated_types::{ self, AuthenticationMessage, Identity, Node, NodeTypes, Prompt };
use crate::generated_types::{
    body::Contents,
    node_content::NodeContent as NodeContentEnum,
    Body,
    Edge,
    Envelope,
    Graph,
    GraphNodeInfo,
    Letter,
    Process,
    UserSettings,
    VerbTypes,
    PromptHistory,
};

use crate::graph::validate_nodes_in_loop;

use colored::*;

use std::sync::Arc;

use crate::graph::{ validate_nodes_in_process, run_execution };
use crate::sqlite_helper_functions::{ fetch_all_nodes, insert_node, update_node };

use crate::SERVER_IDENTITY;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::sync::mpsc::UnboundedSender;

use prost::Message;

use prost::bytes::BytesMut;

use bollard::container::Config;
use bollard::exec::{ CreateExecOptions, StartExecResults };

use bollard::Docker;

// use petgraph::prelude::Bfs;
// use petgraph::algo::toposort;

// use bollard::container::Config;
// use bollard::exec::{ CreateExecOptions, StartExecResults };
// use bollard::Docker;
use bson::doc;
use serde::{ Deserialize, Serialize };
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
    pool: Arc<Pool<SqliteConnectionManager>>
) {
    let runtime_settings: HashMap<LocalServerIdentity, UserSettings> = HashMap::new();
    let mut docker_containers: HashMap<Identity, String> = HashMap::new();

    while let Some(msg) = client_rx.recv().await {
        println!("{} {:?}", "Received a message from the client:".yellow(), msg.1.len());

        // if docker_containers doesn't contain the message identity (msg.0) then create it and add it to the hashmap:

        let mut docker_id: String;

        match docker_containers.get(&msg.0) {
            Some(id) => {
                docker_id = id.clone();
                continue;
            }
            None => {
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

                docker_id = id.clone();

                println!("Created container with id: {}", id);
                docker_containers.insert(msg.0.clone(), id);
            }
        }

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

        println!("{}", "TODO: Collection responses and send them in envelope batch.".red());

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
                                                contents: Some(
                                                    Contents::Node(mutable_node.clone())
                                                ),
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
                Contents::NodesToLoop(nodes_to_loop) => {
                    match verb {
                        VerbTypes::Validate => {
                            let outer_node_info = nodes_to_loop.containing_node_info.clone();

                            let nodes = nodes_to_loop.nodes.clone();
                            match validate_nodes_in_loop(nodes, outer_node_info.unwrap()) {
                                Ok(mutable_node) => {
                                    println!("Nodes validated successfully");

                                    match insert_node(pool.clone(), mutable_node.clone()) {
                                        Ok(_) => {
                                            println!("Node inserted successfully");

                                            // we construct a new letter with the new mutable_node:

                                            let body = Body {
                                                contents: Some(
                                                    Contents::Node(mutable_node.clone())
                                                ),
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

                                            let body = Body {
                                                contents: Some(
                                                    Contents::NodesToLoop(nodes_to_loop.clone())
                                                ),
                                            };

                                            let letter = generated_types::Letter {
                                                verb: VerbTypes::Error as i32,
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
                                    }

                                    // Add process node to the database

                                    // Send process back to the frontend
                                }
                                Err(err) => {
                                    println!("Error validating nodes: {:?}", err);
                                    let body = Body {
                                        contents: Some(
                                            Contents::NodesToLoop(nodes_to_loop.clone())
                                        ),
                                    };

                                    let letter = generated_types::Letter {
                                        verb: VerbTypes::Error as i32,
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
                            match run_execution(execution.clone(), None).await {
                                Ok((execution, _accumulator)) => {
                                    let letter = Letter {
                                        body: Some(Body {
                                            contents: Some(Contents::ExecutionDetails(execution)),
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
                                Err(error_response) => {
                                    let letter = Letter {
                                        body: Some(Body {
                                            contents: Some(
                                                Contents::ExecutionDetails(error_response)
                                            ),
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
        }
    }
}

pub async fn send_message(
    tx: &UnboundedSender<(LocalServerIdentity, tokio_tungstenite::tungstenite::Message)>,
    identity: LocalServerIdentity,
    envelope: Envelope
) {
    let mut buf = BytesMut::new();
    envelope.encode(&mut buf).unwrap();

    println!("{}: {:?}", "Sending message to client".green(), envelope);

    match tx.send((identity, tokio_tungstenite::tungstenite::Message::Binary(buf.to_vec()))) {
        Ok(_) => {}
        Err(e) => {
            println!("Error sending message to client: {:?}", e);
        }
    }
}
