use crate::generated_types::{
    response_object::Object, CrudBundle, ExecutionContext, ExecutionResponse,
    NodeExecutionResponse, NodeTypeNames, ResponseObject,
};
use prost::Message;

// pub fn create_node_response_object(
//     execution_clone: ExecutionContext,
//     node_execution_response: NodeExecutionResponse,
// ) -> ResponseObject {
//     let execution_response: ExecutionResponse = ExecutionResponse {
//         execution_id: execution_clone.execution_id,
//         container_execution_id: execution_clone.return_execution_id,
//         current_node_id: execution_clone
//             .current_node
//             .unwrap()
//             .node_info
//             .unwrap()
//             .id
//             .clone(),
//         current_node_type: NodeTypeNames::Command as i32,
//         response: Some(node_execution_response),
//     };

//     let response_object: ResponseObject = ResponseObject {
//         object: Some(Object::ExecutionResponse(execution_response)),
//     };

//     response_object
// }

pub fn to_u8_vec<M: Message>(message: &M) -> Result<Vec<u8>, prost::EncodeError> {
    // Create a buffer to hold the serialized bytes
    let mut bytes = Vec::new();

    // Serialize the message to the buffer
    message.encode(&mut bytes)?;

    Ok(bytes)
}

pub fn parse_message(message_str: &str) -> Option<CrudBundle> {
    let res: Result<CrudBundle, _> = typed_object_from_base64_string(message_str);

    match res {
        Ok(val) => Some(val),
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

use prost::bytes::Bytes;
fn typed_object_from_base64_string<M: Message + Default>(
    base64_string: &str,
) -> Result<M, Box<dyn std::error::Error>> {
    // Decode the base64 string into bytes
    // let bytes = Engine::decode(base64_string)?;

    let my_bytes = Bytes::from(base64_string.to_owned());

    // Parse the bytes into the specific Prost-generated type
    let message = M::decode(my_bytes)?;

    Ok(message)
}
