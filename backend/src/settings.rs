use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSettings {
    pub openai_api_key: String,
    pub mongo_db_uri: String,
}

impl UserSettings {
    pub fn new() -> Option<UserSettings> {
        let openai_api_key = env::var("OPENAI_API_KEY").unwrap();
        let mongo_db_uri = env::var("MONGO_DB_URI").unwrap();

        Some(UserSettings {
            openai_api_key,
            mongo_db_uri,
        })
    }
}

pub struct RuntimeSettings {
    pub openai_api_key: String,
    pub mongo_db_uri: String,
}
