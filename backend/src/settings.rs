use crate::generated_types::UserSettings;
use std::env;

impl UserSettings {
    pub fn new() -> Option<UserSettings> {
        let openai_api_key = env::var("OPENAI_API_KEY").unwrap();

        Some(UserSettings { openai_api_key })
    }
}
