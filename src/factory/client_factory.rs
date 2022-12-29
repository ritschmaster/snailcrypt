pub use crate::{
	client::{ 
		ClientVersion,
		Client, 
		V1Client,
	},
	config::Config,
	factory::{
		AnalyzerFactory,
		ConfigFactory,
	}
};

#[allow(unused)]
pub struct ClientFactory {
	analyzer_factory: AnalyzerFactory,
	config_factory: ConfigFactory
}

impl ClientFactory {
    #[allow(unused)]
    pub fn new(analyzer_factory: AnalyzerFactory, config_factory: ConfigFactory) -> ClientFactory {
        return ClientFactory { 
			analyzer_factory: analyzer_factory,
			config_factory: config_factory  
		};
    }

    pub fn create(&self, client_version: ClientVersion) -> Box<dyn Client> {        	
		match client_version {
			ClientVersion::V1 => return Box::new(V1Client::new(self.get_analyzer_factory().create(),
        					 	           			           self.get_config_factory().create())),
        }
    }
    
    pub fn get_analyzer_factory(&self) -> &AnalyzerFactory {
		return &self.analyzer_factory;
	}
    
    pub fn get_config_factory(&self) -> &ConfigFactory {
		return &self.config_factory
	}
}
