use crate::env_vars_checker::check_env_variable_valid;
use crate::generated_types::{self, AuthenticationMessage, Identity, Secrets};
use crate::generated_types::{
    body::Contents, Body, Envelope, GraphNodeInfo, Letter, UserSettings, VerbTypes,
};

use crate::generated_types::authentication_message::Body as AuthBody;

use crate::generated_types::Session;

use bollard::container::StartContainerOptions;


use bollard::{API_DEFAULT_VERSION, Docker};

use futures_util::future::TryFutureExt;

use crate::graph::validate_nodes_in_loop;

use colored::*;

use std::sync::Arc;

use crate::graph::{run_execution, validate_nodes_in_process};
use crate::sqlite_helper_functions::{
    authorized, check_if_user_exists, fetch_all_nodes, insert_node, insert_user, update_node,
};

use crate::SERVER_IDENTITY;

use bollard::image::CreateImageOptions;
use futures_util::StreamExt;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tokio::sync::mpsc::UnboundedSender;

use prost::Message;

use prost::bytes::BytesMut;

use bollard::container::Config;

// use bollard::Docker;

use bson::doc;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use tokio::sync::mpsc;

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
    auth_pool: Arc<Pool<SqliteConnectionManager>>,
) {
    // settings will be sent with the session (with the secrets)
    let mut runtime_settings: HashMap<LocalServerIdentity, Option<UserSettings>> = HashMap::new();
    let mut docker_containers: HashMap<String, String> = HashMap::new();
    // let docker = Docker::connect_with_http_defaults().unwrap();


    let docker = Docker::connect_with_http(
        "localhost:2375", 4, API_DEFAULT_VERSION)
        .unwrap();
docker.ping()
.map_ok(|_| Ok::<_, ()>(println!("Connected!")));


    let mut session_ids: Vec<Session> = vec![];

    while let Some(msg) = client_rx.recv().await {
        println!(
            "{} {:?}",
            "Received a message from the client:".yellow(),
            msg.1.len()
        );

        // if docker_containers doesn't contain the message identity (msg.0) then create it and add it to the hashmap:

        let mut docker_id: String = "".to_string();

        println!("Client identity: {:?}", msg.0.name);

        let slice = msg.1.clone().into_data().as_slice().to_vec();

        let envelope: Envelope = match Envelope::decode(&*slice) {
            Ok(val) => val,
            Err(err) => {
                println!("Error decoding message: {:?}", err);
                continue;
            }
        };

        let mut session_found: bool = false;

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
                    session: None,
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

        // check that the envelope has a session, and if not, then the letter MUST be an authentication message (this is a design decision but also like... makes sense, right?
        if envelope.session.is_none() {
            let mut letters = envelope.clone().letters;

            // god save us all... I'm so sorry. I'll fix this with a super cool macro in the future, I promise 😉
            match letters
                .first_mut()
                .unwrap()
                .body
                .clone()
                .unwrap()
                .contents
                .unwrap()
            {
                Contents::AuthenticationMessage(auth) => {
                    println!("{}", "Initiating authentication".green());

                    match check_if_user_exists(&auth_pool, auth.clone()) {
                        Ok(user_exists) => {
                            if user_exists {
                                println!("User exists");
                                match authorized(&auth_pool, auth.clone()) {
                                    Ok(res) => {
                                        if res {
                                            println!(
                                                "User is authorized... Storing their settings"
                                            );

                                            let user_settings = auth.clone().body.unwrap();

                                            match user_settings {
                                                AuthBody::Secrets(secret) => {
                                                    println!("Adding potential user settings to runtime settings");
                                                    runtime_settings.insert(
                                                        msg.0.clone(),
                                                        secret.clone().user_settings.clone(),
                                                    );
                                                }
                                                _ => {
                                                    println!("Secrets not found");
                                                    continue;
                                                }
                                            }
                                        } else {
                                            println!("User not authorized");
                                            continue;
                                        }
                                    }
                                    Err(_) => {
                                        println!("user not authorized");
                                        continue;
                                    }
                                }
                            } else {
                                println!(
                                    "User does not exist. Let's create the account and session"
                                );

                                // Read the allowed emails from the file
                                let allowed_emails =
                                    std::fs::read_to_string("./allowed_emails.txt")
                                        .expect("Failed to read allowed_emails.txt");

                                let allowed_emails: Vec<&str> =
                                    allowed_emails.split("\n").collect();

                                let any_email = check_env_variable_valid("ALLOW_ANY_EMAIL", vec!("TRUE".to_string())).is_ok();

                                match auth.clone().body.unwrap() {
                                    AuthBody::Secrets(secret) => {
                                        println!("Email: {}", secret.email);
                                        println!("Password length: {}", secret.password.len());

                                        // Check if user's email is in the allowed list
                                        if allowed_emails.contains(&secret.email.as_str()) | any_email {
                                            // Create the user and session
                                            match insert_user(&auth_pool, auth.clone()) {
                                                Ok(_) => {
                                                    println!("User created and session started");
                                                    // Here you might want to initiate a session or take other actions
                                                }
                                                Err(e) => {
                                                    println!("Failed to create user: {:?}", e);
                                                    continue;
                                                }
                                            }
                                        } else {
                                            println!("User email is not in the whitelist");
                                            continue;
                                        }
                                    }
                                    _ => {
                                        println!("Secrets not found");
                                        continue;
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            println!("Auth message doesn't contain secret");
                            continue;
                        }
                    }
                    // we know that the sender/receiver for the envelope is set. We will add these to the session

                    // create session here and add it to session id:
                    let new_session = Session {
                        session_id: uuid::Uuid::new_v4().to_string(),
                        client_identity: envelope.clone().sender,
                        backend_identity: envelope.clone().receiver,
                    };

                    let mut response_envelope: Envelope = Envelope {
                        sender: Some(envelope.clone().receiver.unwrap()),
                        receiver: Some(envelope.clone().sender.unwrap()),
                        letters: Vec::new(),
                        verification_id: envelope.clone().verification_id,
                        session: Some(new_session.clone()),
                    };

                    let mut response_letter: Letter = Letter {
                        verb: VerbTypes::Acknowledge as i32,
                        body: None,
                    };

                    let mut response_body: Body = Body { contents: None };

                    let mut response_contents: Contents =
                        Contents::AuthenticationMessage(generated_types::AuthenticationMessage {
                            body: Some(generated_types::authentication_message::Body::Session(
                                new_session.clone(),
                            )),
                        });

                    response_body.contents = Some(response_contents);

                    response_letter.body = Some(response_body);

                    response_envelope.letters.push(response_letter);

                    // put the session into the session_ids vec
                    session_ids.push(new_session.clone());

                    println!(
                        "{}: {:?}",
                        "added the session to the session vector".green(),
                        new_session.clone()
                    );

                    send_message(&tx, msg.0.clone(), response_envelope).await;
                    continue;
                }
                _ => {
                    println!(
                        "{}",
                        "There needs to be an auth message in the case that there is not yet a session".red()
                    );
                    continue;
                }
            }
        } else {
            let test_session = envelope.clone().session.unwrap();
            // check that this session is in the session_ids vec
            for session in session_ids.clone() {
                if session.session_id == test_session.session_id {
                    session_found = true;
                }
            }
            if !session_found {
                println!("{}", "Session not found".red());
                continue;
            }
        }

        // If we get to this point, we can assume that the user is verified

        // only create the container AFTER all of the verifications have been performed.
        match docker_containers.get(&msg.0.name) {
            Some(id) => {
                docker_id = id.clone();
                // continue;
            }
            None => {
                const IMAGE: &str = "alpine";

                let alpine_config = Config {
                    image: Some(IMAGE),
                    tty: Some(true),
                    attach_stdin: Some(true),
                    attach_stdout: Some(true),
                    attach_stderr: Some(true),
                    open_stdin: Some(true),
                    ..Default::default()
                };

                match docker
                    .create_container::<&str, &str>(None, alpine_config.clone())
                    .await
                {
                    Ok(container) => {
                        println!("Created container with id: {:?}", container.id);
                        docker_id = container.id.clone();
                        docker_containers.insert(msg.0.clone().name.to_string(), docker_id.clone());
                    }
                    Err(err) => {
                        println!(
                            "Error creating container: {:?}. Let's try pulling the image:",
                            err
                        );
                        let options = CreateImageOptions {
                            from_image: "alpine",
                            ..Default::default()
                        };

                        let mut stream = docker.create_image(Some(options), None, None);
                        while let Some(output) = stream.next().await {
                            println!("{:?}", output);
                        }
                    }
                }
            }
        }

        // loop through the letters and handle each one
        for letter in envelope.clone().letters {
            // println!("Message content: {:?}", letter);
            let verb: VerbTypes = VerbTypes::try_from(letter.verb).unwrap();
            let sender: Identity = envelope.clone().sender.unwrap();
            let receiver: generated_types::Identity = envelope.clone().receiver.unwrap();
            let wrapped_content = letter.body.clone();
            let verification_id = envelope.clone().verification_id;
            let session: Session = envelope.clone().session.unwrap();
            let user_settings = match runtime_settings.get(&msg.0.clone()) {
                Some(settings) => settings.clone(),
                None => {
                    println!(
                        "{} {:?}",
                        "No user settings found for user: ".red(),
                        msg.0.clone()
                    );
                    continue;
                }
            };

            let content: Contents = match wrapped_content {
                None => {
                    println!("{} {:?}", "No contents found:".red(), letter);
                    continue; // We should probably log this later... But we don't want to interrupt the message processing loop
                }
                Some(body) => body.contents.unwrap(),
            };

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
                                        session: Some(session.clone()),
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
                                        session: Some(session.clone()),
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
                                session: Some(session.clone()),
                            };

                            match fetch_all_nodes(pool.clone()) {
                                Ok(nodes) => {
                                    for node in &nodes {
                                        // println!("Found node: {:?}", node);

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
                        _ => {
                            println!(
                                "{}",
                                "Session already set... Authentication should only be handled when when there is no session... Might add a Halt verb that could be handled here".red()
                            );
                        }
                    } // Closing the match verb
                }
                Contents::UserSettings(_user_settings) => {}
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
                                                session: Some(session.clone()),
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
                                                session: Some(session.clone()),
                                            };

                                            send_message(&tx, msg.0.clone(), response_object).await;
                                        }
                                        Err(err) => {
                                            println!("Error inserting node: {:?}", err);

                                            let body = Body {
                                                contents: Some(Contents::NodesToLoop(
                                                    nodes_to_loop.clone(),
                                                )),
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
                                                session: Some(session.clone()),
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
                                        contents: Some(Contents::NodesToLoop(
                                            nodes_to_loop.clone(),
                                        )),
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
                                        session: Some(session.clone()),
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
                            // start up the server before running the execution as the recursive function is not allowed to send between async threads.
                            match docker
                                .start_container(&docker_id, None::<StartContainerOptions<String>>)
                                .await
                            {
                                Ok(res) => {
                                    println!("Container started: {:?}", res);
                                }
                                Err(err) => {
                                    println!("Container not started: {:?}", err);
                                }
                            }

                            // make sure the openai_api_key is set
                            match user_settings {
                                Some(settings) => {
                                    let settings: Arc<UserSettings> = Arc::new(settings.clone());

                                    match run_execution(
                                        execution.clone(),
                                        None,
                                        Some(docker_id.clone()),
                                        &docker,
                                        settings,
                                    )
                                    .await
                                    {
                                        Ok((execution, _accumulator)) => {
                                            let letter = Letter {
                                                body: Some(Body {
                                                    contents: Some(Contents::ExecutionDetails(
                                                        execution,
                                                    )),
                                                }),

                                                verb: VerbTypes::Acknowledge as i32,
                                            };

                                            let envelope = Envelope {
                                                letters: vec![letter],
                                                sender: Some(receiver.clone()),
                                                receiver: Some(sender.clone()),
                                                verification_id: verification_id.clone(),
                                                session: Some(session.clone()),
                                            };

                                            send_message(&tx, msg.0.clone(), envelope).await;
                                        }
                                        Err(error_response) => {
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
                                                session: Some(session.clone()),
                                            };

                                            send_message(&tx, msg.0.clone(), envelope).await;
                                        }
                                    }
                                }
                                None => {
                                    println!("api key not found.. request this from the user?");
                                    continue;
                                }
                            };
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
    envelope: Envelope,
) {
    let mut buf = BytesMut::new();
    envelope.encode(&mut buf).unwrap();

    // println!("{}: {:?}", "Sending message to client".green(), envelope);
    println!("{}", "Sending message to client".green());



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
