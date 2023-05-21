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
    util::Analyzer,
};

use std::rc::Rc;

/**
 * This factory produces clients.
 *
 * # Examples
 *
 * ## Directly specifying the client version
 *
 * ```
 * let analyzer_factory: factory::AnalyzerFactory = factory::AnalyzerFactory::new();
 * let config_factory: factory::ConfigFactory = factory::ConfigFactory::new();
 * let client_factory: factory::ClientFactory = factory::ClientFactory::new(analyzer_factory, config_factory);
 * let client: Box<dyn client::Client> = client_factory.create(client::ClientVersion::V1);

 * ```
 *
 * ## Using the client version extracted from the plaintext
  * ```
 * let plaintext: String = String::from("hello world");
 *
 * let analyzer_factory: factory::AnalyzerFactory = factory::AnalyzerFactory::new();
 * let config_factory: factory::ConfigFactory = factory::ConfigFactory::new();
 * let client_factory: factory::ClientFactory = factory::ClientFactory::new(analyzer_factory, config_factory);
 * let client: Box<dyn client::Client> = client_factory.create();

 * ```
 */
#[allow(unused)]
pub struct ClientFactory {
    analyzer: Rc<dyn Analyzer>,
    config: Rc<dyn Config>
}

impl<'a> ClientFactory {
    #[allow(unused)]
    pub fn new(analyzer: Rc<dyn Analyzer>,
            config: Rc<dyn Config>) -> ClientFactory {
        return ClientFactory { 
			analyzer: analyzer,
            config: config,
		};
    }

    /**
     * Create a new client object using a specific version.
     */
    pub fn create(&self, client_version: ClientVersion) -> Rc<dyn Client> {
		match client_version {
			ClientVersion::V1 
			=> 
			return Rc::new(V1Client::new(Rc::clone(self.get_analyzer()),
                                    Rc::clone(self.get_config()))),
        	ClientVersion::V2
        	=> 
        	return Rc::new(V2Client::new(Rc::clone(self.get_analyzer()),
                                        Rc::clone(self.get_config())))
        }
    }
    
    /**
     * Get the analyzer.
     */
    pub fn get_analyzer(&self) -> &Rc<dyn Analyzer> {
		return &self.analyzer;
	}
    
    /**
     * Get the configuraton.
     */
    pub fn get_config(&self) -> &Rc<dyn Config> {
		return &self.config
	}
}
