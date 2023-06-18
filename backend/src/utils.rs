use crate::domain::create_node;
use crate::domain::CreateNode;
use crate::domain::InitializeProject;
use crate::domain::NodeType;
use crate::domain::{MessageTypes, Node, Process, Prompt};

pub fn parse_message(message_str: &str) -> Option<MessageTypes> {
    use serde_json::Value;

    let value: Value = match serde_json::from_str(message_str) {
        Ok(val) => val,
        Err(_) => return None, // or handle this error as you see fit
    };

    if let Some(obj) = value.as_object() {
        if let Some(create_action_value) = obj.get("create_action") {
            if let Some(create_action_obj) = create_action_value.as_object() {
                let prompt = Prompt {
                    input_variables: create_action_obj
                        .get("input_variables")
                        .and_then(|v| v.as_array())
                        .unwrap_or(&vec![])
                        .iter()
                        .map(|v| v.as_str().unwrap_or("").to_string())
                        .collect(),
                    output_variables: create_action_obj
                        .get("output_variables")
                        .and_then(|v| v.as_array())
                        .unwrap_or(&vec![])
                        .iter()
                        .map(|v| v.as_str().unwrap_or("").to_string())
                        .collect(),
                    name: create_action_obj
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    prompt: create_action_obj
                        .get("prompt")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    system: create_action_obj
                        .get("system")
                        .and_then(|v| v.as_str())
                        .map(|v| v.to_string()),
                };

                return Some(MessageTypes::CreateNode(CreateNode {
                    node: create_node(NodeType::Prompt(prompt)),
                }));
            }
        }

        if let Some(create_process_value) = obj.get("create_process") {
            if let Some(create_process_obj) = create_process_value.as_object() {
                let process = Process {
                    max_iterations: create_process_obj
                        .get("max_iterations")
                        .and_then(|v| v.as_u64())
                        .map(|v| v as u32),
                    is_loop: create_process_obj
                        .get("is_loop")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                    name: create_process_obj
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    graph: create_process_obj
                        .get("graph")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    description: create_process_obj
                        .get("description")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    topological_order: create_process_obj
                        .get("topological_order")
                        .and_then(|v| v.as_array())
                        .unwrap_or(&vec![])
                        .iter()
                        .map(|v| v.as_str().unwrap_or("").to_string())
                        .collect(),
                    output_variable: create_process_obj
                        .get("output_variable")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                };

                return Some(MessageTypes::CreateNode(CreateNode {
                    node: create_node(NodeType::Process(process)),
                }));
            }
        }

        if let Some(initialize_project) = obj.get("initial_message") {
            return Some(MessageTypes::InitializeProject(InitializeProject {
                initial_message: "intialized".to_string(),
            }));
        }

        println!("Could not parse message: {}", message_str);
    }

    None
}
