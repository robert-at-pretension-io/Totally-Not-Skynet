use crate::generated_types::{
    ExecutionContext,
    ExecutionResponse,
    NodeExecutionResponse,
    ResponseObject,
    CrudBundle,
    NodeTypeName,
};

pub fn create_node_response_object(
    execution_clone: ExecutionContext,
    node_execution_response: NodeExecutionResponse
) -> ResponseObject {
    let execution_response: ExecutionResponse = ExecutionResponse {
        execution_id: execution_clone.execution_id,
        container_execution_id: execution_clone.container_execution_id,
        current_node_id: execution_clone.current_node._id.clone().unwrap().to_string(),
        current_node_type: NodeTypeName::Command,
        response: node_execution_response,
    };

    let response_object: ResponseObject = ResponseObject::ExecutionContext(execution_response);

    response_object
}

pub fn parse_message(message_str: &str) -> Option<CrudBundle> {
    match serde_json::from_str(message_str) {
        Ok(val) => {
            return val;
        }
        Err(err) => {
            println!("Could not parse message: {}", err);
            return None;
        } // or handle this error as you see fit
    };
}
