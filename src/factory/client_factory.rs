/******************************************************************************
  This file is part of snailcrypt.

  Copyright 2023 Richard Bäck <richard.baeck@snailcrypt.com>

  Permission is hereby granted, free of charge, to any person obtaining a copy
  of this software and associated documentation files (the "Software"), to deal
  in the Software without restriction, including without limitation the rights
  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
  copies of the Software, and to permit persons to whom the Software is
  furnished to do so, subject to the following conditions:

  The above copyright notice and this permission notice shall be included in all
  copies or substantial portions of the Software.

  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
  SOFTWARE.
*******************************************************************************/

pub use crate::{
	client::{ 
		ClientVersion,
		Client, 
		V1Client,
		V2Client,
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
			ClientVersion::V1 
			=> 
			return Box::new(V1Client::new(self.get_analyzer_factory().create(),
        	   			           self.get_config_factory().create())),
        	ClientVersion::V2
        	=> 
        	return Box::new(V2Client::new(self.get_analyzer_factory().create(),
        								  self.get_config_factory().create()),
        					)
        }
    }
    
    pub fn get_analyzer_factory(&self) -> &AnalyzerFactory {
		return &self.analyzer_factory;
	}
    
    pub fn get_config_factory(&self) -> &ConfigFactory {
		return &self.config_factory
	}
}
