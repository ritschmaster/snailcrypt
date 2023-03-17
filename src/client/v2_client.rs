use crate::{    
	client::{
		Client,
		ClientVersion,
		ClientEncryptArg,
		ClientDecryptResultSuccess,
		ClientDecryptResultFailure,		
		V1Client,
	},
	config::Config,
	util::Analyzer,
};

use chrono::{
    DateTime,
    FixedOffset,
};

#[allow(unused)]
pub struct V2Client {
	v1_client: V1Client,
}

impl V2Client {
    #[allow(unused)]
    pub fn new(analyzer: Box<dyn Analyzer>, config: Box<dyn Config>) -> V2Client {
        return V2Client { 
        	v1_client: V1Client::new(analyzer, config) 
        };
    }
}

impl Client for V2Client {
    fn encrypt(&self, args: &ClientEncryptArg) -> Result<String, String> {
    	let encrypt_result = self.v1_client.encrypt(&ClientEncryptArg { 
	    	plaintext: args.plaintext.clone(),
	    	lockdate: args.lockdate, 
	    	hint: String::from(""),
    	});
    	
    	if encrypt_result.is_err() {
    		return Err(encrypt_result.unwrap_err())
    	}
    	
    	let encrypted = encrypt_result.unwrap();
    	let mut cipher_comp_vec: Vec<&str> = encrypted
    		.split_terminator(':')
    		.collect(); 
    	cipher_comp_vec.remove(0);
    	
    	let corrected_ciphertext: String = cipher_comp_vec
    		.join(":");
    		
    		
    	let mut final_ciphertext: String = self.get_client_version()
    		.to_string();
    	final_ciphertext.push_str(":");
    	final_ciphertext.push_str(corrected_ciphertext.as_str());
    	final_ciphertext.push_str(":");
    	final_ciphertext.push_str(base64::encode(args.hint.as_str()).as_str());
    	
		Ok(final_ciphertext)
    }

    fn decrypt(&self, ciphertext: &str) 
    	-> Result<ClientDecryptResultSuccess, ClientDecryptResultFailure> {
    	let mut cipher_comp_vec: Vec<&str> = ciphertext.split_terminator(':').collect();

        if cipher_comp_vec.len() != 4 {
            return Err(ClientDecryptResultFailure { 
        		error_message: String::from("Cipher is invalid. It must consist of 4 components separted by a colon."),
        		hint: String::from(""),
        	});
        }
        
        let hint_base64_result = base64::decode(cipher_comp_vec[3]);
        if hint_base64_result.is_err() {
        	return Err(ClientDecryptResultFailure { 
        		error_message: hint_base64_result.unwrap_err().to_string(), 
        		hint: String::from(""),
        	});
        }        
        
        let hint_result = String::from_utf8(hint_base64_result.unwrap());
        if hint_result.is_err() {
        	return Err(ClientDecryptResultFailure { 
        		error_message: hint_result.unwrap_err().to_string(), 
        		hint: String::from(""),
        	});
        }
        
        let hint = hint_result.unwrap();

		cipher_comp_vec.remove(3);
		let corrected_ciphertext = cipher_comp_vec.join(":");
    
    	let decrypt_result = self.v1_client.decrypt(corrected_ciphertext.as_str());
    	if decrypt_result.is_err() {
    		return Err(ClientDecryptResultFailure { 
        		error_message: decrypt_result.unwrap_err().error_message.clone(), 
        		hint,
        		});
    	}
    	    	
    	Ok(ClientDecryptResultSuccess { 
    		plaintext: decrypt_result.unwrap().plaintext.clone(),
    		hint,
		})    	    	
    }
    
    fn lockdate_from_snailcrypt_cipher(&self, ciphertext: &str) -> Result<DateTime<FixedOffset>, String> {
        let cipher_comp_vec: Vec<&str> = ciphertext.split_terminator(':').collect();

        if cipher_comp_vec.len() != 4 {
            // Err("Cipher is invalid. It must consist of two components separted by a colon.")
            panic!("Cipher is invalid. It must consist of 4 components separted by a colon.");
        }

        let lockdate: DateTime<FixedOffset> = 
        	DateTime::parse_from_str(
				String::from_utf8(
					base64::decode(cipher_comp_vec[1])
                 	.unwrap_or_else(|error| {
                    	panic!("Error: {:?}", error);
                    }))
				.unwrap_or_else(|error| {
   					panic!("Error: {:?}", error);
				})
				.as_str(),
				self.get_datetime_format())
			.unwrap_or_else(|error| {
            	panic!("Error: {:?}", error);
			});

        Ok(lockdate)
    }
        
    fn get_datetime_format(&self) -> &str {
    	self.v1_client.get_datetime_format()
    }
    
	fn get_client_version(&self) -> ClientVersion {
		return ClientVersion::V2
	}
}