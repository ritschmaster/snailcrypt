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

use crate::{
    client::ClientVersion,
    util::Analyzer,
};

/// The default implementation of an analyzer.
#[allow(unused)]
pub struct DefaultAnalyzer {
}

impl DefaultAnalyzer {
    #[allow(unused)]
    pub fn new() -> DefaultAnalyzer {
        return DefaultAnalyzer { };
    }
}

impl Analyzer for DefaultAnalyzer {
    fn get_version(&self, ciphertext: &str) -> Result<ClientVersion, String> {
		if ciphertext.is_empty() {
			panic!("Cipher is invalid. It must at least contain something.");
		}	
	
		let cipher_comp_vec: Vec<&str> = ciphertext.split_terminator(':').collect();

        let version: String = String::from(cipher_comp_vec[0]);
        
        match self.str_to_version(version.as_str()) {
			Ok(version) => Ok(version),
			Err(err)    => Err(err),
		}
    }
    
    fn str_to_version(&self, client_version: &str) -> Result<ClientVersion, String> {
		match client_version {
			"1" => Ok(ClientVersion::V1),
			"2" => Ok(ClientVersion::V2),			
			_   => Err(format!("Unknown client version: {}", client_version)),
		}
	}
}
