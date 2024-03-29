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

use std::{
	fmt,
	string,
};

use chrono::{
    DateTime,
    FixedOffset,
};

/// Enumeration for the available client versions. This can be used to identify the client object you are using.
#[derive(Debug, PartialEq, Eq)]
pub enum ClientVersion {
    V1,
    V2,
	V3,
}

/// Input parameter structure to encrypt something using a client object.
pub struct ClientEncryptArg {
	pub plaintext: String,
	pub lockdate: DateTime<FixedOffset>,
	pub hint: String,
	pub filename: String,
}

/// Result parameter structure on success after decrypting something using a client object.
pub struct ClientDecryptResultSuccess {
	pub plaintext: String,
	pub hint: String,
	pub filename: String,
}

/// This method will just print the plain text for a decryption result.
impl string::ToString for ClientDecryptResultSuccess {
	fn to_string(&self) -> String {
    	return self.plaintext
    		.clone();
	}
}

/// This method will print the plain text and the hint (if available) for a decryption result.
impl fmt::Debug for ClientDecryptResultSuccess {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	f.debug_struct("ClientDecryptResultSuccess")
    		.field("plaintext", &self.plaintext)
    		.field("hint", &self.hint)
    		.finish()
	}
}

/// Result parameter structure on failure after decrypting something using a client object.
pub struct ClientDecryptResultFailure {
	pub error_message: String,
	pub hint: String,
	pub filename: String,
}

/// This method will just print the error message.
impl string::ToString for ClientDecryptResultFailure {
	fn to_string(&self) -> String {
    	return self.error_message
    		.clone();
	}
}

/// This method will print the error message and the hint (if available) for a decryption result.
impl fmt::Debug for ClientDecryptResultFailure {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	f.debug_struct("ClientDecryptResultFailure")
    		.field("error_message", &self.error_message)
    		.field("hint", &self.hint)
    		.finish()
	}
}

/// This method enables stringifying the client version easily.
impl fmt::Display for ClientVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ClientVersion::V1 => write!(f, "1"),
			ClientVersion::V2 => write!(f, "2"),
			ClientVersion::V3 => write!(f, "3"),
		}        
    }
}

/// This is the main trait of this library. It will provide everything needed to perform an encryption or an decryption using the services available on snailcrypt.com.
pub trait Client {
    /// Encrypt a plain text.
    fn encrypt(&self, args: &ClientEncryptArg) 
    	-> 
    	Result<String, String>;

    /// Decrypt a cipher text.
    fn decrypt(
    	&self, 
    	ciphertext: &str
    	) 
    	->
    	Result<ClientDecryptResultSuccess, ClientDecryptResultFailure>;
    
    /// Extract the lockdate from a cipher text.
    fn lockdate_from_snailcrypt_cipher(
    	&self, 
    	ciphertext: &str
    	)
    	->
    	Result<DateTime<FixedOffset>, String>;
        
    /// Get the supported date time format of this client.
    fn get_datetime_format(&self) -> &str;
    
    /// Get the client version of this client.
	fn get_client_version(&self) -> ClientVersion;    
}
