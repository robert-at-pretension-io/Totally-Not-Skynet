use crate::settings::UserSettings;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Action {
    prompt: String,
    input_variables: Vec<String>,
    output_variables: Vec<String>,
    name: String,
    system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Process {
    name: String,
    graph: String,
    topological_order: Vec<String>,
    description: String,
    output_variable: String,
    is_loop: bool,
    max_iterations: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conditional {
    _id: Option<ObjectId>,
    system_variables: HashMap<String, String>,
    statement: String,
    options: HashMap<String, ObjectId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodeType {
    Action(Action),
    Process(Process),
    Conditional(Conditional),
    Command(Command),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    _id: Option<ObjectId>,
    pub node_type: NodeType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RunCommand {
    command: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct InitializeProject {
    initial_message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    action_id: String,
    response_text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateAction {
    pub action: Action,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandOutput {
    command: String,
    success: bool,
    response: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAction {
    pub create_action: Action,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateProcess {
    pub create_process: Process,
}

// Used for the websocket messages
#[derive(Serialize, Deserialize, Debug)]
pub enum MessageTypes {
    InitializeProject(InitializeProject), // Add more types here
    SetUserSettings(UserSettings),
    HandleNode(Node),
    UpdateAction(UpdateAction),
    CreateAction(CreateAction),
    CreateProcess(CreateProcess),
}
