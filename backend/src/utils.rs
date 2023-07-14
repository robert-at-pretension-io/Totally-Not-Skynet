use crate::domain::create_node;
use crate::domain::CreateNode;
use crate::domain::InitializeProject;
use crate::domain::NodeType;
use crate::domain::{CrudBundle, Process, Prompt};
use crate::settings::UserSettings;

pub fn parse_message(message_str: &str) -> Option<CrudBundle> {

    match serde_json::from_str(message_str) {
        Ok(val) => return val,
        Err(err) => {
            println!("Could not parse message: {}", err)
            return None}, // or handle this error as you see fit
    };

}
