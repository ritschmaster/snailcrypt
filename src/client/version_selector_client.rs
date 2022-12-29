use crate::client::Client;

use chrono::{
    DateTime,
    FixedOffset,
};
use serde_json::Value;

#[allow(unused)]
pub struct VersionSelectorClient {
}

impl VersionSelectorClient {
    #[allow(unused)]
    pub fn new(api_url: &str) -> VersionSelectorClient {
        return VersionSelectorClient {  };
    }
}

impl Client for VersionSelectorClient {
    fn encrypt(&self, plaintext: &str, lockdate: DateTime<FixedOffset>) -> Result<String, &'static str> {
        Ok(String::from("hello world"))
    }

    fn decrypt(&self, ciphertext: &str) -> Result<String, &'static str> {
        Ok(String::from("hello world"))
    }

    fn get_api_url(&self) -> &str {
        return "hello world";
    }


    fn get_datetime_format(&self) -> &str {
        return "%Y-%m-%dT%H:%M:%S%z";
    }
}
