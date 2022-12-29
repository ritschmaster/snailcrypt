use crate::config::Config;

pub struct DefaultConfig {
    api_url: String,	
}

impl DefaultConfig {
    #[allow(unused)]
    pub fn new() -> DefaultConfig {
		return DefaultConfig { api_url: String::from("https://api.snailcrypt.com") };
    }
}

impl Config for DefaultConfig {
	fn get_api_url(&self) -> &str {
        return self.api_url.as_str();
    }
}