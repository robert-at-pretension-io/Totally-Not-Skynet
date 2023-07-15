use crate::settings::UserSettings;
use bson::oid::ObjectId;
use serde::{ Deserialize, Serialize };
use std::{ collections::HashMap };

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prompt {
    pub prompt: String,
    pub system: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Process {
    pub graph: String,
    pub topological_order: Vec<String>,
    pub initial_variables: Vec<String>,
    pub is_loop: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conditional {
    pub system_variables: HashMap<String, String>,
    pub statement: String,
    pub options: HashMap<String, ObjectId>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodeType {
    Prompt(Prompt),
    Process(Process),
    Conditional(Conditional),
    Command(Command),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub _id: Option<ObjectId>,
    pub name: String,
    pub type_name: NodeTypeName,
    pub node_content: NodeType,
    pub description: String,
    pub input_variables: Vec<String>,
    pub output_variables: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NodeTypeName {
    Prompt,
    Process,
    Conditional,
    Command,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Command {
    pub command: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct InitializeProject {
    pub initial_message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VerbTypeNames {
    POST,
    PUT,
    PATCH,
    DELETE,
    GET,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSettings {
    pub openai_api_key: String,
    pub mongo_db_uri: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecutionContext {
    pub topological_order: Vec<String>,
    pub current_node: Node,
    pub variables: HashMap<String, String>,
    pub execution_id: String,
    pub return_execution_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CrudBundleObject {
    Node(Node),
    InitialMessage(InitialMessage),
    UserSettings(UserSettings),
    ExecutionContext(ExecutionContext),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CrudBundle {
    pub verb: VerbTypeNames,
    pub object: CrudBundleObject,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandOutput {
    command: String,
    success: bool,
    response: String,
}
