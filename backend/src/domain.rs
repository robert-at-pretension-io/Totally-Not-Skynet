use crate::settings::UserSettings;
use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Action {
    _id: Option<ObjectId>,
    prompt: String,
    input_variables: Vec<String>,
    output_variables: Vec<String>,
    name: String,
    system: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Process {
    _id: Option<ObjectId>,
    name: String,
    graph: String,
    topological_order: Vec<String>,
    description: String,
    output_variable: String,
    is_loop: bool,
    max_iterations: Option<u32>,
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
    Prompt(Prompt),
    Conditional(Conditional),
    ExecuteCommand(RunCommand),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    _id: Option<ObjectId>,
    node_type: NodeType,
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
    action: Action,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandOutput {
    command: String,
    success: bool,
    response: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAction {
    create_action: Action,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateProcess {
    create_process: Process,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageTypes {
    InitializeProject(InitializeProject), // Add more types here
    SetUserSettings(UserSettings),
    HandleNode(Node),
    UpdateAction(UpdateAction),
    CreateAction(CreateAction),
    CreateProcess(CreateProcess),
}
