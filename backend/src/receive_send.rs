use crate::generated_types::{
    crud_bundle,
    CrudBundle,
    GraphNodeInfo,
    ResponseObject,
    UserSettings,
    VerbTypeNames,
    Graph,
    ValidateNodes,
    response_object,
    ValidateNodesResponse,
    Edge,
};

use colored::*;
use petgraph::Direction;
use petgraph::prelude::DiGraph;

use std::sync::Arc;

use crate::generated_types::response_object::Object::{
    Node,
    ValidateNodesResponse as ValidateNodesResponseEnum,
};

// use crate::utils::parse_message;

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

pub async fn start_message_sending_loop(
    // docker: Docker,
    tx: UnboundedSender<(Identity, tokio_tungstenite::tungstenite::Message)>,
    mut client_rx: mpsc::Receiver<(Identity, tokio_tungstenite::tungstenite::Message)>,
    pool: Arc<Pool<SqliteConnectionManager>>
) {
    let mut runtime_settings: HashMap<Identity, UserSettings> = HashMap::new();

    while let Some(msg) = client_rx.recv().await {
        println!("{} {:?}", "Received a message from the client:".yellow(), msg);

        // let received_message: Option<CrudBundle> = parse_message(&msg.1);

        // println!("message data: {:?}",msg.1.into_data());

        let slice = msg.1.clone().into_data().as_slice().to_vec();
        let message_contents: CrudBundle;

        match CrudBundle::decode(&*slice) {
            Ok(val) => {
                message_contents = val;
            }
            Err(err) => {
                println!("Error decoding message: {:?}", err);
                continue;
            }
        }

        // if received_message.is_none() {
        //     print!("Received an invalid message from the client: {}", msg.1);
        //     continue;
        // } else {
        //     message_contents = received_message.unwrap();
        //     println!("Received a parsed message from the client: {:?}", message_contents);
        // }

        let verb: VerbTypeNames = VerbTypeNames::from_i32(message_contents.verb).unwrap();

        match message_contents.object {
            Some(crud_bundle::Object::Node(node)) => {
                match verb {
                    VerbTypeNames::Post => {
                        let mut mutable_node = node.clone();

                        println!("Creating node: {:?}", mutable_node);

                        let new_node_info = GraphNodeInfo {
                            id: uuid::Uuid::new_v4().to_string(),
                            description: node.clone().node_info.unwrap().description.clone(),
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
                                    println!("Found node: {:?}", node);

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
            Some(crud_bundle::Object::ExecutionContext(_execution_context)) => {
                match verb {
                    _ => {
                        todo!("Handle execution context");
                    }
                }
            }
            Some(crud_bundle::Object::ValidateNodes(node_container)) => {
                match verb {
                    VerbTypeNames::Post => {
                        // generate maximal graph from nodes (based on input_variables and output_variables)
                        println!("Validating nodes");
                        println!("Validating nodes for user: {:?}", msg.0);
                        println!("need to return a Graph object");

                        // create a petgraph digraph:

                        let nodes = node_container.nodes;
                        println!("Number of nodes: {:?}", nodes.len());

                        // From the nodes and their dependencies, we need to populate a petgraph graph so that we can run the transitive reduction algorithm on it
                        // This will give a "minimal" graph that has the same topological order as the larger graph
                        // This will allow the user to have a visualization of the graph that is not cluttered with unnecessary nodes and is easy to understand the topological order

                        let mut graph = DiGraph::new();
                        println!("Initialized empty graph");

                        let mut new_nodes: Vec<GraphNodeInfo> = Vec::new();

                        // let mut node_indices;
                        let mut node_indices: HashMap<
                            String,
                            petgraph::graph::NodeIndex
                        > = HashMap::new();

                        for node in &nodes {
                            let node_info = node.node_info.clone().unwrap();

                            println!("Processing node: {:?}", node_info.id);
                            new_nodes.push(node_info);
                            let node_index = graph.add_node(node.clone());
                            node_indices.insert(node.node_info.clone().unwrap().id, node_index);
                        }

                        let mut mut_pruned_graph = graph.clone();

                        println!("All nodes added to graph");

                        // Add edges based on input_variables and output_variables
                        for node in &nodes {
                            let node_index = node_indices[&node.node_info.clone().unwrap().id];
                            println!(
                                "Adding edges for node: {:?}, Input Vars: {:?}",
                                node.node_info.clone().unwrap().id,
                                node.input_variables
                            );
                            for input_var in &node.input_variables {
                                // Find nodes that output this input_var
                                // For demonstration, using the same list of nodes
                                println!("Checking for input_var: {:?}", input_var);
                                for other_node in &nodes {
                                    println!(
                                        "Other node: {:?}, Output Vars: {:?}",
                                        other_node.node_info.clone().unwrap().id,
                                        other_node.output_variables
                                    );
                                    if other_node.output_variables.contains(input_var) {
                                        let other_node_index =
                                            node_indices[&other_node.node_info.clone().unwrap().id];
                                        println!(
                                            "Found matching output_var in node: {:?}",
                                            other_node.node_info.clone().unwrap().id
                                        );
                                        graph.add_edge(other_node_index, node_index, ());
                                    }
                                }
                            }
                        }
                        println!("All edges added");

                        println!("{}", "Remove the excess edges here".red());

                        let top_sort = petgraph::algo::toposort(&graph, None).unwrap();

                        let adjacency_list =
                            petgraph::algo::tred::dag_to_toposorted_adjacency_list::<
                                _,
                                petgraph::graph::DefaultIx
                            >(&graph, &top_sort);

                        // The output is the pair of the transitive reduction and the transitive closure.
                        let (transative_reduct, _) =
                            petgraph::algo::tred::dag_transitive_reduction_closure::<
                                _,
                                petgraph::graph::DefaultIx
                            >(&adjacency_list.0);

                        // The graph should have the same nodes but different edges.

                        // for all of the edges in transivite_reduct, add that edge to the mut_pruned_graph

                        transative_reduct.edge_indices().for_each(|edge| {
                            // how to get the source and target nodes from the edge?
                            let (sourceIndex, targetIndex) = transative_reduct
                                .edge_endpoints(edge)
                                .unwrap();
                            mut_pruned_graph.add_edge(sourceIndex.into(), targetIndex.into(), ());
                        });

                        let mut new_edges: Vec<Edge> = Vec::new();

                        // Count and print the number of edges
                        // let edge_count = rebuilt_graph.raw_edges().len();
                        // println!("Total number of raw edges: {}", edge_count);

                        mut_pruned_graph
                            .raw_edges()
                            .iter()
                            .for_each(|edge| {
                                let source_node = graph.node_weight(edge.source()).unwrap();
                                let target_node = graph.node_weight(edge.target()).unwrap();

                                let new_edge: Edge = Edge {
                                    source: source_node.node_info.clone(),
                                    target: target_node.node_info.clone(),
                                };

                                new_edges.push(new_edge);
                            });

                        let new_graph = Graph {
                            nodes: new_nodes,
                            edges: new_edges,
                        };

                        let mut starting_nodes = Vec::new();

                        for node in graph.node_indices() {
                            if graph.neighbors_directed(node, Direction::Incoming).count() == 0 {
                                println!("Node with no incoming edges: {:?}", graph[node]);
                                starting_nodes.push(graph[node].clone());
                            }
                        }

                        let validate_nodes_response = ValidateNodesResponse {
                            errors: Vec::new(),
                            graph: Some(new_graph),
                            topological_order: Vec::new(),
                        };

                        let response_object = ResponseObject {
                            object: Some(ValidateNodesResponseEnum(validate_nodes_response)),
                        };

                        send_message(&tx, msg.0.clone(), response_object).await;
                    }
                    _ => {
                        println!("Verb not supported for node validation: {:?}", verb);
                    }
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

pub async fn send_message(
    tx: &UnboundedSender<(Identity, tokio_tungstenite::tungstenite::Message)>,
    identity: Identity,
    message: ResponseObject
) {
    let mut buf = BytesMut::new();
    message.encode(&mut buf).unwrap();

    println!("{}: {:?}", "Sending message to client".green(), message);

    match tx.send((identity, tokio_tungstenite::tungstenite::Message::Binary(buf.to_vec()))) {
        Ok(_) => {}
        Err(e) => {
            println!("Error sending message to client: {:?}", e);
        }
    }
}
