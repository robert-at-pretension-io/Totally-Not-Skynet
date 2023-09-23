use crate::generated_types::CrudBundle;
use prost::Message;
use colored::*;

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
            println!("{} {}", "Could not parse message.".red(), err);

            return None;
        }
    }
}

use prost::bytes::Bytes;
fn typed_object_from_base64_string<M: Message + Default>(
    base64_string: &str
) -> Result<M, Box<dyn std::error::Error>> {
    let my_bytes = Bytes::from(base64_string.to_owned());

    // Parse the bytes into the specific Prost-generated type
    let message = M::decode(my_bytes);

    match message {
        Ok(val) => {
            return Ok(val);
        }
        Err(err) => {
            println!("{} {}", "Could not parse message.".red(), err);
            println!("{}", "Attempting to parse using alternate parsing function".yellow());

            let message = M::decode_length_delimited(&mut base64::decode(base64_string)?[..]);
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
