pub use crate::config::{ 
	Config, 
	DefaultConfig,
};

#[allow(unused)]
pub struct ConfigFactory {
}

impl ConfigFactory {
    #[allow(unused)]
    pub fn new() -> ConfigFactory {
        return ConfigFactory { };
    }

    pub fn create(&self) -> Box<dyn Config> {
        return Box::new(DefaultConfig::new());
    }
}
