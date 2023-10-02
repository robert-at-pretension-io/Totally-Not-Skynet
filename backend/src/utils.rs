use crate::generated_types::CrudBundle;
use petgraph::{ prelude::DiGraph, adj::UnweightedList, stable_graph::IndexType };
use prost::Message;
use colored::*;
use base64;

pub fn to_u8_vec<M: Message>(message: &M) -> Result<Vec<u8>, prost::EncodeError> {
    // Create a buffer to hold the serialized bytes
    let mut bytes = Vec::new();

    // Serialize the message to the buffer
    message.encode(&mut bytes)?;

    Ok(bytes)
}

// pub fn parse_message<M: Message + Default>(message: M) -> Option<CrudBundle> {
//     println!("{}", "calling parse_message".yellow());
//     // let res: Result<CrudBundle, _> = typed_object_from_base64_string(message_str);

//     // message.

//     let res = Message::decode(message);

//     match res {
//         Ok(val) => Some(val),
//         Err(err) => {
//             println!("{} {}", "Could not parse message.".red(), err);

//             return None;
//         }
//     }
// }

use prost::bytes::Bytes;
fn typed_object_from_base64_string<M: Message + Default>(
    base64_string: &str
) -> Result<M, Box<dyn std::error::Error>> {
    println!("{}", "calling typed_object_from_base64_string".yellow());

    let my_bytes = Bytes::from(base64_string.to_owned());
    // let my_bytes = base64::decode(base64_string.to_owned()).unwrap().into_buf();

    println!("The bytes are: {:?}", my_bytes.clone());

    // Parse the bytes into the specific Prost-generated type
    let message = M::decode(my_bytes.clone());

    match message {
        Ok(val) => {
            return Ok(val);
        }
        Err(err) => {
            println!("{} {}", "Could not parse message.".red(), err);
            println!("{}", "Attempting to parse using alternate parsing function".yellow());

            let message = M::decode_length_delimited(my_bytes.clone());
            match message {
                Ok(val) => {
                    return Ok(val);
                }
                Err(err) => {
                    println!(
                        "{} {}",
                        "Could not parse message using alternative method.".red(),
                        err
                    );

                    return Err(Box::new(err));
                }
            }
        }
    }
}
