use crate::{
    client::ClientVersion,
    util::Analyzer,
};

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
