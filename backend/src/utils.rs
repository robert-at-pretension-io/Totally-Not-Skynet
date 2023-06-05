use crate::domain::{Action, Process, MessageTypes, InitializeProject, Node };
use crate::settings::{UserSettings};
crate::domain::CreateAction;

pub fn parse_message(message_str: &str) -> Option<MessageTypes> {
    use serde_json::Value;
    let value: Value = match serde_json::from_str(message_str) {
        Ok(val) => val,
        Err(_) => return None, // or handle this error as you see fit
    };

    if let Some(obj) = value.as_object() {
        if let Some(create_action_value) = obj.get("create_action") {
            if let Some(create_action_obj) = create_action_value.as_object() {
                let action = Action {
                    _id: None, // Assuming you have changed your struct field to `_id`
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
                        .unwrap_or("")
                        .to_string(),
                };
                return Some(MessageTypes::CreateAction(CreateAction {
                    create_action: action,
                }));
            }
        }
        if let Some(create_process_value) = obj.get("create_process") {
            if let Some(create_process_obj) = create_process_value.as_object() {
                let process = Process {
                    _id: None, // Assuming you have changed your struct field to `_id`
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
                return Some(MessageTypes::CreateProcess(CreateProcess {
                    create_process: process,
                }));
            }
        }
    }

    if let Ok(msg) = serde_json::from_str::<InitializeProject>(message_str) {
        return Some(MessageTypes::InitializeProject(msg));
    }

    if let Ok(msg) = serde_json::from_str::<UserSettings>(message_str) {
        return Some(MessageTypes::SetUserSettings(msg));
    }

    if let Ok(msg) = serde_json::from_str::<Node>(message_str) {
        return Some(MessageTypes::HandleNode(msg));
    }

    if let Ok(msg) = serde_json::from_str::<UpdateAction>(message_str) {
        return Some(MessageTypes::UpdateAction(msg));
    }

    println!("Could not parse message: {}", message_str);

    None
}