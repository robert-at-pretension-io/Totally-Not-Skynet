use std::fmt;
use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::{json, Value as JsonValue};
use serde_json::Result;



// types used for sending messages to openai
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Choice {
    pub message: ChatMessage,
    pub finish_reason: String,
    pub index: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ChatCompletion {
    id: String,
    object: String,
    created: u64,
    model: String,
    usage: Usage,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
}

impl fmt::Display for ChatMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} : {}", self.role, self.content)
    }
}

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Role {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
}

impl fmt::Display for Role {
    // this is the implementation of the fmt::Display trait
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::System => write!(f, "system"),
            Role::User => write!(f, "user"),
            Role::Assistant => write!(f, "assistant"),
        }
    }
}

pub async fn get_openai_completion(messages: Vec<ChatMessage>, api_key: String) -> Result<String> {
    // Define the URL for the API endpoint
    let url = "https://api.openai.com/v1/chat/completions";

    // Define the initial request body
    let mut body: JsonValue = json!({
        "model": "gpt-3.5-turbo",
        "messages": messages,
        "temperature": 0.7
    });

    // Set up the headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );

    // Create an HTTP client
    let client = reqwest::Client::new();

    let mut response_string = String::new();

    // Loop to make repeated API requests
    loop {
        // Make the HTTP POST request asynchronously
        let response = client
            .post(url)
            .headers(headers.clone())
            .body(body.to_string())
            .send()
            .await
            .unwrap();

        // Deserialize the response JSON into the ChatCompletion struct
        let chat_completion: ChatCompletion =
            serde_json::from_str(&response.text().await.unwrap())?;

        // Print the result
        println!("{:#?}", chat_completion);

        // Check if the finish_reason is "stop"
        if let Some(choice) = chat_completion.choices.first() {
            if choice.finish_reason == "stop" {
                // If the finish_reason is "stop", exit the loop
                response_string = choice.message.content.clone();
                break;
            } else {
                // If the finish_reason is not "stop", update the request body
                // to include the assistant's response and make another request
                if let JsonValue::Array(messages) = &mut body["messages"] {
                    messages.push(json!(choice.message));
                }
            }
        } else {
            // If there are no choices, exit the loop
            break;
        }
    }

    Ok(response_string)
}
