use serde::{Deserialize, Serialize};
use std::env;



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