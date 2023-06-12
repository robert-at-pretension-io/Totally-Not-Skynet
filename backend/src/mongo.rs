use crate::domain::{Action, Process};
use bson::{doc};
use futures_util::StreamExt;
use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Prompt {
    action_id: String,
    system: String,
    prompt_text: String,
}

pub async fn get_actions_and_processes(db: &mongodb::Database) -> (Vec<Action>, Vec<Process>) {
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

pub async fn get_nodes(db: &mongodb::Database) -> Vec<crate::domain::Node> {
    let node_collection = db.collection::<crate::domain::Node>("nodes");

    let mut node_cursor = node_collection.find(None, None).await.unwrap();

    let mut nodes = Vec::new();

    while let Some(node) = node_cursor.next().await {
        if let Ok(node) = node {
            nodes.push(node);
        }
    }

    nodes
}

pub async fn return_db(db_uri: String) -> mongodb::Database {
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
