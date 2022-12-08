use crate::client::Client;

use std::io::Read;

use chrono::{
    DateTime,
    FixedOffset,
};
use curl::easy::Easy;
use serde_json::Value;
use openssl::{
    rsa::{
        Rsa,
        Padding,
    },
    pkey::{
        Public,
        Private,
    }
};

const PLAINTEXT_CHUNK_SIZE: i32 = 126;

#[allow(unused)]
pub struct DefaultClient {
    api_url: String,
}

impl DefaultClient {
    #[allow(unused)]
    pub fn new(api_url: &str) -> DefaultClient {
        return DefaultClient { api_url: String::from(api_url) };
    }

    fn send_lockdate_request(&self, lockdate: DateTime<FixedOffset>) -> Result<Value, &'static str> {
        /***********************************************************************
         * Setup HTTP post input data
         */
        let mut input_string: String = String::from("{\"lock_date\":\"");
        input_string.push_str(lockdate.format(self.get_datetime_format()).to_string().as_str());
        input_string.push_str("\"}");
        let mut input_str = input_string.as_bytes();

        /***********************************************************************
         * Setup HTTP post output data
         */
        let mut output_vector: Vec<u8> = Vec::with_capacity(512);

        /***********************************************************************
         * Retrieve the URL
         */
        let api_url = self.get_api_url();
        let api_url_keys = api_url.to_string() + "/keys";

        /***********************************************************************
         * Perform HTTP POST which will eventually fill output_vector
         */
        {
            /*******************************************************************
             * Basic setup of curl
             */
            let mut handle = Easy::new();
            handle.url(&api_url_keys.as_str()).unwrap();
            handle.post(true).unwrap_or_else(|error| {
                panic!("Error: {:?}", error);
            });
            handle.post_field_size(input_str.len() as u64).unwrap_or_else(|error| {
                panic!("Error: {:?}", error);
            });

            /*******************************************************************
            + Set function to send data
             */
            let mut transfer = handle.transfer();
            transfer.read_function(|buffer| {
                Ok(input_str.read(buffer).unwrap_or(0))
            }).unwrap_or_else(|error| {
                panic!("Error: {:?}", error);
            });

            /*******************************************************************
            + Set function to receive data
             */
            transfer.write_function(|buffer| {
                output_vector.extend_from_slice(buffer);

                Ok(buffer.len())
            }).unwrap_or_else(|error| {
                panic!("Error: {:?}", error);
            });

            /*******************************************************************
             * Perform POST
             */
            transfer.perform().unwrap_or_else(|error| {
                panic!("Error {:?}", error);
            });
        }

        /***********************************************************************
         * Prase received JSON
         */
        let output_object: Value = serde_json::from_slice(&output_vector).unwrap_or_else(|error| {
            panic!("Error {:?}", error);
        });

        Ok(output_object)
    }

    fn get_public_key(&self, lockdate: DateTime<FixedOffset>) -> Result<Rsa<Public>, &'static str> {
        /***********************************************************************
         * Extract public key attribute
         */
        let output_object: Value = self.send_lockdate_request(lockdate)
                                       .unwrap_or_else(|error| {
                                           panic!("Error {:?}", error);
                                       });

        /***********************************************************************
         * Extract public key attribute
         */
        let public_key_str: String = String::from(output_object["public_key"]
                                                  .as_str()
                                                  .unwrap_or_else(|| {
                                                      panic!("Error ");
                                                  }))
                                    .replace("'", "");

        /***********************************************************************
         * Create public key object using the extracted public key
         */
        let public_key: Rsa<Public> = Rsa::public_key_from_pem(public_key_str.as_bytes()).unwrap_or_else(|error| {
            panic!("Error {:?}", error);
        });

        Ok(public_key)
    }

    fn get_private_key(&self, lockdate: DateTime<FixedOffset>) -> Result<Rsa<Private>, &'static str> {
        /***********************************************************************
         * Extract public key attribute
         */
        let output_object: Value = self.send_lockdate_request(lockdate)
                                       .unwrap_or_else(|error| {
                                           panic!("Error {:?}", error);
                                       });

        /***********************************************************************
         * Extract public key attribute
         */
        let private_key_str: String = String::from(output_object["private_key"]
                                                  .as_str()
                                                  .unwrap_or_else(|| {
                                                      panic!("Error ");
                                                  }))
                                    .replace("'", "");

        /***********************************************************************
         * Create private key object using the extracted private key
         */
        let private_key: Rsa<Private> = Rsa::private_key_from_pem(private_key_str.as_bytes()).unwrap_or_else(|error| {
            panic!("Error {:?}", error);
        });

        Ok(private_key)
    }

    fn to_snailcrypt_cipher(&self, ciphertext: &str, lockdate: &str) -> String {
        let mut snailcrypt_cipher: String = String::from(lockdate);
        snailcrypt_cipher.push(':');
        snailcrypt_cipher.push_str(ciphertext);

        return snailcrypt_cipher;
    }

    fn lockdate_from_snailcrypt_cipher(&self, ciphertext: &str) -> Result<DateTime<FixedOffset>, &'static str> {
        let cipher_comp_vec: Vec<&str> = ciphertext.split_terminator(':').collect();

        if cipher_comp_vec.len() != 2 {
            // Err("Cipher is invalid. It must consist of two components separted by a colon.")
            panic!("Cipher is invalid. It must consist of two components separted by a colon.");
        }

        let lockdate: DateTime<FixedOffset> = DateTime::parse_from_str(String::from_utf8(base64::decode(cipher_comp_vec[0])
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

    fn cipher_from_snailcrypt_cipher(&self, ciphertext: &str) -> Result<String, &'static str> {
        let cipher_comp_vec: Vec<&str> = ciphertext.split_terminator(':').collect();

        if cipher_comp_vec.len() != 2 {
            // Err("Cipher is invalid. It must consist of two components separted by a colon.")
            panic!("Cipher is invalid. It must consist of two components separted by a colon.");
        }

        let cipher: String = String::from(cipher_comp_vec[1]);

        Ok(cipher)
    }
}

impl Client for DefaultClient {
    fn encrypt(&self, plaintext: &str, lockdate: DateTime<FixedOffset>) -> Result<String, &'static str> {
        /***********************************************************************
         * Get the public key for the requested lockdate
         */
        let public_key: Rsa<Public> = self.get_public_key(lockdate).unwrap_or_else(|error| {
            panic!("Error {:?}", error);
        });

        /***********************************************************************
         * Encrypt the plaintext
         */
        let ciphertext_chunk_size: i32 = public_key.size() as i32;

        let mut cipher_vector: Vec<u8> = Vec::new();
        let cipher_vector_len: usize = (((plaintext.len() as f64 / PLAINTEXT_CHUNK_SIZE as f64).ceil() as i32) * ciphertext_chunk_size) as usize;
        cipher_vector.resize(cipher_vector_len, 0);

        let mut i: i32 = 0;
        while i * PLAINTEXT_CHUNK_SIZE < plaintext.len() as i32 {
            let plaintext_cur_start: usize = (i * PLAINTEXT_CHUNK_SIZE) as usize;
            let mut plaintext_cur_end: usize = plaintext_cur_start + PLAINTEXT_CHUNK_SIZE as usize;
            if plaintext_cur_end > plaintext.len() {
                plaintext_cur_end = plaintext.len();
            }

            let plaintext_slice = &
                (plaintext.as_bytes())[plaintext_cur_start .. plaintext_cur_end];

            let cipher_cur_start: usize = (i * ciphertext_chunk_size) as usize;
            let cipher_cur_end: usize = cipher_cur_start + ciphertext_chunk_size as usize;
            let cipher_vector_slice = &mut
                (cipher_vector.as_mut_slice())[cipher_cur_start .. cipher_cur_end];

            public_key
                .public_encrypt(plaintext_slice,
                                cipher_vector_slice,
                                Padding::PKCS1_OAEP)
                .unwrap_or_else(|error| {
                    panic!("Error {:?}", error);
                });

            i += 1;
        }

        /***********************************************************************
         * Encode the cipher_vector to base64
         */
        let cipher_string: String = base64::encode(cipher_vector.as_slice());

        /***********************************************************************
         * Encode the lockdate
         */
        let lockdate_string: String = base64::encode(lockdate.format(self.get_datetime_format()).to_string());

        Ok(self.to_snailcrypt_cipher(cipher_string.as_str(), lockdate_string.as_str()))
    }

    fn decrypt(&self, ciphertext: &str) -> Result<String, &'static str> {
        /***********************************************************************
         * Extract the lockdate from the ciphertext
         */
        let lockdate: DateTime<FixedOffset> = self.lockdate_from_snailcrypt_cipher(ciphertext)
            .unwrap_or_else(|error| {
                panic!("Error {:?}", error);
            });

        /***********************************************************************
         * Get the private key for the lockdate
         */
        let private_key: Rsa<Private> = self
            .get_private_key(lockdate)
            .unwrap_or_else(|error| {
                panic!("Error {:?}", error);
            });

        /***********************************************************************
         * Decrypt ciphertext
         */
		let cipher_vector = base64::decode(self
                                            .cipher_from_snailcrypt_cipher(ciphertext)
                                            .unwrap_or_else(|error| {
                                                panic!("Error {:?}", error);
                                            }))
                             .unwrap_or_else(|error| {
                                 panic!("Error {:?}", error);
                             });                             

        let ciphertext_chunk_size: i32 = private_key.size() as i32;

        let mut plaintext_vector: Vec<u8> = Vec::new();
        let plaintext_vector_len: usize = ciphertext.len();
        plaintext_vector.resize(plaintext_vector_len, 0);

         let mut i: i32 = 0;
         let mut plaintext_cur_actual_end: usize = 0;
         while i * ciphertext_chunk_size < cipher_vector.len() as i32 {
             let cipher_cur_start: usize = (i * ciphertext_chunk_size) as usize;
             let cipher_cur_end: usize = cipher_cur_start + ciphertext_chunk_size as usize;
             let cipher_vector_slice = &
                 (cipher_vector.as_slice())[cipher_cur_start .. cipher_cur_end];
                 
             /*
			  * A plaintext chunk may have a different size then a ciphertext chunk. Thererfore we
			  * start at a different position but end at the same place as the ciphertext chunk.
			  * The consequence of this is that the plaintext_vector may have a smaller length
			  * at the end as the sum of the ciphertext chunks.
			  */    
             let plaintext_cur_start: usize = plaintext_cur_actual_end;
             let plaintext_cur_end: usize = cipher_cur_end;
             let plaintext_vector_slice = &mut
             	(plaintext_vector.as_mut_slice())[plaintext_cur_start .. plaintext_cur_end];
             	
			 private_key
	            .private_decrypt(cipher_vector_slice,
	                             plaintext_vector_slice,
	                             Padding::PKCS1_OAEP)
	            .unwrap_or_else(|error| {
	                panic!("Error {:?}", error);
	            });             	

             i += 1;
             plaintext_cur_actual_end += PLAINTEXT_CHUNK_SIZE as usize;
         }
//        private_key
//            .private_decrypt(base64::decode(self
//                                            .cipher_from_snailcrypt_cipher(ciphertext)
//                                            .unwrap_or_else(|error| {
//                                                panic!("Error {:?}", error);
//                                            }))
//                             .unwrap_or_else(|error| {
//                                 panic!("Error {:?}", error);
//                             })
//                             .as_slice(),
//                             plaintext_vector.as_mut_slice(),
//                             Padding::PKCS1_OAEP)
//            .unwrap_or_else(|error| {
//                panic!("Error {:?}", error);
//            });

        let mut end_pos: usize = plaintext_vector.len();
        for (pos, elem) in plaintext_vector.iter().enumerate() {
            if *elem as char == '\0' {
                end_pos = pos;
                break;
            }
        }

        plaintext_vector.resize(end_pos, 0);

        Ok(String::from_utf8(plaintext_vector)
           .unwrap_or_else(|error| {
               panic!("Error {:?}", error);
           }))
    }

    fn get_api_url(&self) -> &str {
        return self.api_url.as_str();
    }


    fn get_datetime_format(&self) -> &str {
        return "%Y-%m-%dT%H:%M:%S%z";
    }
}
