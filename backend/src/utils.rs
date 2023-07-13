use crate::domain::create_node;
use crate::domain::CreateNode;
use crate::domain::InitializeProject;
use crate::domain::NodeType;
use crate::domain::{MessageTypes, Node, Process, Prompt};
use crate::settings::UserSettings;

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

                let name = create_action_obj
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();

                return Some(MessageTypes::CreateNode(CreateNode {
                    node: create_node(NodeType::Prompt(prompt), name),
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
                    graph: create_process_obj
                        .get("graph")
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
                };

                let name = create_process_obj
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();

                return Some(MessageTypes::CreateNode(CreateNode {
                    node: create_node(NodeType::Process(process), name),
                }));
            }
        }

        if let Some(initialize_project) = obj.get("initial_message") {
            return Some(MessageTypes::InitializeProject(InitializeProject {
                initial_message: "intialized".to_string(),
            }));
        }

        if let Some(user_settings) = obj.get("openai_api_key") {
            return Some(MessageTypes::SetUserSettings(UserSettings {
                openai_api_key: "".to_string(),
                mongo_db_uri: "".to_string(),
            }));
        }
        println!("Could not parse message: {}", message_str);
    }

    None
}
