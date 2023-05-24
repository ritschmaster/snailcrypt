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

/// This factory produces clients.
///
/// # Examples
///
/// ```
/// use snailcrypt::{
///     client,
///     config,
///     factory,
///     util,
/// };
///
/// use std::rc::Rc;
///
/// let analyzer_factory: factory::AnalyzerFactory = factory::AnalyzerFactory::new();
/// let analyzer: Rc<dyn util::Analyzer> = analyzer_factory.create();
///
/// let config_factory: factory::ConfigFactory = factory::ConfigFactory::new();
/// let config: Rc<dyn config::Config> = config_factory.create();
///
/// let client_factory: factory::ClientFactory = factory::ClientFactory::new(Rc::clone(&analyzer),
///                             Rc::clone(&config));
/// let client: Rc<dyn client::Client> = client_factory.create(client::ClientVersion::V1);
/// ```
///
/// ## Directly specifying the client version
///
/// ```
/// use snailcrypt::{
///     client,
///     config,
///     factory,
///     util,
/// };
///
/// use std::rc::Rc;
///
/// let cipher: String = String::from("1:MjAyMi0xMS0xOVQxNzowMDowMCswMTAw:bVoPtqSST34ojbXQHEdTfQuvCgI7Ed/SsBLSNczVhoCSmMcpJNv3/rAGomn+hNJihmzOu7RQXDTNEnkewV4TXrMGuWqvfmCIAPTTQnuUkqLimuL8WD2Nu8LY2LaPMf3G1Q9JiRb+dd7lmboppgOd9ssPciAXTiI0NkJ4SawBW/PVWOuEFAWDs2MBkPT6oxbJrNha5L0lHDpgHMTP9HsdVf3gh9GiKuwQFtaZ3WXKTKUnOPALz3QkcLOspFHP+UuOUuZn4OrkxpWGbTFqS00NROwT4a5V0vbY/Ag+RYJtd9Pk3UsTT4QNUSj1vQ81X27tC6+B8gXxaVGWRynIgYn5wQ==");
///
/// let analyzer_factory: factory::AnalyzerFactory = factory::AnalyzerFactory::new();
/// let analyzer: Rc<dyn util::Analyzer> = analyzer_factory.create();
///
/// let config_factory: factory::ConfigFactory = factory::ConfigFactory::new();
/// let config: Rc<dyn config::Config> = config_factory.create();
///
/// let client_factory: factory::ClientFactory = factory::ClientFactory::new(Rc::clone(&analyzer),
///                             Rc::clone(&config));
/// let client: Rc<dyn client::Client> = client_factory.create(analyzer
///     .get_version(cipher
///         .as_str())
///     .unwrap_or_else(|error| {
///         panic!("Error: {:?}", error);
///     }));
/// ```
#[allow(unused)]
pub struct ClientFactory {
    analyzer: Rc<dyn Analyzer>,
    config: Rc<dyn Config>
}

impl ClientFactory {
    #[allow(unused)]
    pub fn new(analyzer: Rc<dyn Analyzer>,
            config: Rc<dyn Config>) -> ClientFactory {
        return ClientFactory { 
			analyzer: analyzer,
            config: config,
		};
    }

    /// Create a new client object using a specific version.
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
    
    /// Get the analyzer.
    pub fn get_analyzer(&self) -> &Rc<dyn Analyzer> {
		return &self.analyzer;
	}
    
    /// Get the configuraton.
    pub fn get_config(&self) -> &Rc<dyn Config> {
		return &self.config
	}
}
