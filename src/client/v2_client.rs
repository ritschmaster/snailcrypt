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

use std::rc::Rc;

use chrono::{
    DateTime,
    FixedOffset,
};

#[allow(unused)]
pub struct V2Client {
	v1_client: V1Client,
}

/// This object implements the version 2 of snailcrypt strings. The version 2 allows encrypting an arbitary string until a specified date. It also allows the inclusion of a hint string which is not encrypted.
impl V2Client {
    #[allow(unused)]
    pub fn new(analyzer: Rc<dyn Analyzer>, config: Rc<dyn Config>) -> V2Client {
        return V2Client { 
        	v1_client: V1Client::new(analyzer, config) 
        };
    }
}

impl Client for V2Client {
    fn encrypt(&self, args: &ClientEncryptArg) -> Result<String, String> {
		/**********************************************************************
		 * Use the v1 client to encrypt the string.
		 */
     	let encrypt_result = self.v1_client.encrypt(&ClientEncryptArg { 
	    	plaintext: args.plaintext.clone(),
	    	lockdate: args.lockdate, 
	    	hint: String::from(""),
        	filename: String::from(""),
    	});

		/**********************************************************************
		 * Exit out on error
		 */
    	if encrypt_result.is_err() {
    		return Err(encrypt_result.unwrap_err())
    	}
    	
		/**********************************************************************
		 * Split the final cipher text from v1 to manipulate it later on
		 */
     	let encrypted = encrypt_result.unwrap();
    	let mut cipher_comp_vec: Vec<&str> = encrypted
    		.split_terminator(':')
    		.collect(); 

		/**********************************************************************
		 * Remove the version indicator
		 */   	
     	cipher_comp_vec.remove(0);
 
 		/**********************************************************************
		 * Use the cipher text from v1 without the version indicator
		 */
    	let corrected_ciphertext: String = cipher_comp_vec
    		.join(":");
    		
		/**********************************************************************
		 * Build up the cipher text for v2 using the result from v1
		 */
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
		
		/**********************************************************************
		 * Verify the cipher text 
		 */
        if cipher_comp_vec.len() != 4 {
            return Err(ClientDecryptResultFailure { 
        		error_message: String::from("Cipher is invalid. It must consist of 4 components separted by a colon."),
        		hint: String::from(""),
        		filename: String::from(""),
        	});
        }
  		
		/**********************************************************************
		 * Try to decode the BASE64 string of the hint and exit out on error
		 */
        let hint_base64_result = base64::decode(cipher_comp_vec[3]);
        if hint_base64_result.is_err() {
        	return Err(ClientDecryptResultFailure { 
        		error_message: hint_base64_result.unwrap_err().to_string(), 
        		hint: String::from(""),
        		filename: String::from(""),
        	});
        }        
 		
		/**********************************************************************
		 * Try to create an UTF-8 string from the decoded BASE64 hint and 
		 * exit out on error        
		 */
        let hint_result = String::from_utf8(hint_base64_result.unwrap());
        if hint_result.is_err() {
        	return Err(ClientDecryptResultFailure { 
        		error_message: hint_result.unwrap_err().to_string(), 
        		hint: String::from(""),
        		filename: String::from(""),
        	});
        }
        
		/**********************************************************************
  		 * Finally extracts the hint
  		 */      
        let hint = hint_result.unwrap();

		cipher_comp_vec.remove(3);
		let corrected_ciphertext = cipher_comp_vec.join(":");
    
    	let decrypt_result = self.v1_client.decrypt(corrected_ciphertext.as_str());
    	if decrypt_result.is_err() {
    		return Err(ClientDecryptResultFailure { 
        		error_message: decrypt_result.unwrap_err().error_message.clone(), 
        		hint,
        		filename: String::from(""),
        	});
    	}
    	    	
    	Ok(ClientDecryptResultSuccess { 
    		plaintext: decrypt_result.unwrap().plaintext.clone(),
    		hint,
        	filename: String::from(""),
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
