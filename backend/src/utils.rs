use crate::generated_types::{
    ExecutionContext,
    ExecutionResponse,
    NodeExecutionResponse,
    ResponseObject,
    CrudBundle,
};

pub fn create_node_response_object(
    execution_clone: ExecutionContext,
    node_execution_response: NodeExecutionResponse
) -> ResponseObject {
    let execution_response: ExecutionResponse = ExecutionResponse {
        execution_id: execution_clone.execution_id,
        container_execution_id: execution_clone.return_execution_id,
        current_node_id: execution_clone.current_node.unwrap().id.clone(),
        current_node_type: NodeTypeName::Command,
        response: Some(node_execution_response),
    };

    let response_object: ResponseObject = ResponseObject {
        object: Some(execution_response),
    };

    response_object
}

use prost::Message;
// use base64::{ encode };
use base64::Engine::encode;

pub fn to_base64_string<M: Message>(message: &M) -> Result<String, prost::EncodeError> {
    // Create a buffer to hold the serialized bytes
    let mut bytes = Vec::new();

    // Serialize the message to the buffer
    message.encode(&mut bytes)?;

    // Encode the bytes as a base64 string
    let base64_string = encode(&bytes);

    Ok(base64_string)
}

pub fn parse_message(message_str: &str) -> Option<CrudBundle> {
    let res: Result<CrudBundle, _> = typed_object_from_base64_string(message_str);

    match res {
        Ok(val) => { Some(val) }
        Err(err) => {
            println!("Could not parse message: {}", err);
            return None;
        }
    }

    // match serde_json::from_str(message_str) {
    //     Ok(val) => {
    //         return val;
    //     }
    //     Err(err) => {
    //         println!("Could not parse message: {}", err);
    //         return None;
    //     } // or handle this error as you see fit
    // };
}

use base64::Engine::decode;

fn typed_object_from_base64_string<M: Message + Default>(
    base64_string: &str
) -> Result<M, Box<dyn std::error::Error>> {
    // Decode the base64 string into bytes
    let bytes = decode(base64_string)?;

    // Parse the bytes into the specific Prost-generated type
    let message = M::decode(&*bytes)?;

    Ok(message)
}
