use crate::settings::UserSettings;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prompt {
    pub prompt: String,
    input_variables: Vec<String>,
    output_variables: Vec<String>,
    name: String,
    pub system: Option<String>,
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
    system_variables: HashMap<String, String>,
    statement: String,
    options: HashMap<String, ObjectId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodeType {
    // Action(Action),
    Prompt(Prompt),
    Process(Process),
    Conditional(Conditional),
    Command(Command),
    // perhaps add flow control nodes (such as those required for loops)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub _id: Option<ObjectId>,
    pub type_name: String,
    pub node_content: NodeType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Edge {
    _id: Option<ObjectId>,
    pub a: ObjectId,
    pub b: ObjectId,
}

pub fn create_node(node: NodeType) -> Node {
    Node {
        _id: Some(bson::oid::ObjectId::new()),
        type_name: match node {
            NodeType::Prompt(_) => "Prompt".to_string(),
            NodeType::Process(_) => "Process".to_string(),
            NodeType::Conditional(_) => "Conditional".to_string(),
            NodeType::Command(_) => "Command".to_string(),
        },
        node_content: node,
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Command {
    pub command: String,
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
    pub action: Prompt,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateNode {
    pub node: Node,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandOutput {
    command: String,
    success: bool,
    response: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAction {
    pub create_action: Prompt,
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
    UpdateNode(UpdateNode),
    CreateNode(UpdateNode),
    // CreateAction(CreateAction),
    // CreateProcess(CreateProcess),
}
