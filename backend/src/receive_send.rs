use crate::generated_types::{
    crud_bundle,
    CrudBundle,
    GraphNodeInfo,
    ResponseObject,
    UserSettings,
    VerbTypeNames,
};

use std::sync::Arc;

use crate::generated_types::response_object::Object::Node;

use crate::utils::parse_message;

use crate::sqlite_helper_functions::{ insert_node, update_node, fetch_all_nodes };

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::sync::mpsc::UnboundedSender;

use prost::Message;

use prost::bytes::BytesMut;

// use bollard::container::Config;
// use bollard::exec::{ CreateExecOptions, StartExecResults };
// use bollard::Docker;
use bson::doc;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use tokio::sync::mpsc;
// use tokio_tungstenite::tungstenite::Message;

// create a "models" type that can be used to select the model to use
// it should be one of a couple of strings: "gpt-4", "gpt3.5-turbo", etc
// const DEFAULT_MODEL: &str = "gpt-4";

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
    tx: UnboundedSender<(Identity, tokio_tungstenite::tungstenite::Message)>,
    mut client_rx: mpsc::Receiver<(Identity, String)>,
    pool: Arc<Pool<SqliteConnectionManager>>
) {
    let mut runtime_settings: HashMap<Identity, UserSettings> = HashMap::new();
    // let mut messages_thus_far: HashMap<Identity, Vec<String>> = HashMap::new();
    // let mut docker_containers: HashMap<Identity, String> = HashMap::new();

    // startup the docker container here
    // let docker = Docker::connect_with_local_defaults().unwrap();
    //read messages from the client
    while let Some(msg) = client_rx.recv().await {
        println!("Received a message from the client: {:?}", msg);

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
                match verb {
                    VerbTypeNames::Post => {
                        let mut mutable_node = node.clone();

                        let new_node_info = GraphNodeInfo {
                            id: uuid::Uuid::new_v4().to_string(),
                            name: node.node_info.unwrap().name.clone(),
                        };

                        // create a uuid for the node:
                        mutable_node.node_info = Some(new_node_info);

                        // get_sqlite_db is a function that returns a connection to the sqlite db

                        //insert the node into the db
                        match insert_node(pool.clone(), mutable_node.clone()) {
                            Ok(_) => {
                                println!("Node inserted successfully");
                                let response_object = ResponseObject {
                                    object: Some(Node(mutable_node.clone())),
                                };

                                send_message(&tx, msg.0.clone(), response_object).await;
                            }
                            Err(err) => {
                                println!("Error inserting node: {:?}", err);
                            }
                        }
                    }
                    VerbTypeNames::Put => {
                        let updated_node = node.clone();

                        update_node(pool.clone(), &updated_node).unwrap();

                        let response_object: ResponseObject = ResponseObject {
                            object: Some(Node(updated_node)),
                        };

                        send_message(&tx, msg.0.clone(), response_object).await;
                    }
                    _ => {
                        println!("Verb not supported for node: {:?}", verb);
                    }
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

                        println!("Get nodes, settings, etc from db!");

                        match fetch_all_nodes(pool.clone()) {
                            Ok(nodes) => {
                                for node in &nodes {
                                    send_message(&tx, msg.0.clone(), ResponseObject {
                                        object: Some(Node(node.clone())),
                                    }).await;
                                }
                            }
                            Err(err) => {
                                println!(
                                    "Have the following errors when attempting to pull nodes from sqlite : {:?}",
                                    err
                                );
                            }
                        }

                        // let db_uri = runtime_settings.get(&msg.0).unwrap().mongo_db_uri.clone();

                        // let db = return_db(db_uri).await;

                        // let nodes = get_nodes(&db).await;

                        // println!("Found the following nodes: {:?}", nodes);

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
            Some(crud_bundle::Object::ExecutionContext(_execution_context)) =>
                match verb {
                    _ => {
                        todo!("Handle execution context");
                    }
                }

            None => {
                println!("odd...");
                println!(
                    "This probably means that the websocket connection has closed... Should remove it from the identity hash"
                );
            }
        }
    }
}
use crate::utils::to_u8_vec;

pub async fn send_message(
    tx: &UnboundedSender<(Identity, tokio_tungstenite::tungstenite::Message)>,
    identity: Identity,
    message: ResponseObject
) {
    let mut buf = BytesMut::new();
    message.encode(&mut buf).unwrap();

    match tx.send((identity, tokio_tungstenite::tungstenite::Message::Binary(buf.to_vec()))) {
        Ok(_) => {}
        Err(e) => {
            println!("Error sending message to client: {:?}", e);
        }
    }

    // match to_u8_vec(&message) {
    //     Ok(u8_vec) => {
    //         // //convert u8_vec to string
    //         // let send_string = String::from_utf8(u8_vec).unwrap();

    //         match tx.send((identity, Message::Binary(u8_vec))) {
    //             Ok(_) => {}
    //             Err(e) => {
    //                 println!("Error sending message to client: {:?}", e);
    //             }
    //         }
    //     }
    //     Err(e) => {
    //         println!("Error encoding message: {:?}", e);
    //     }
    // }
}
