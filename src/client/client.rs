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

#[derive(Debug, PartialEq, Eq)]
pub enum ClientVersion {
    V1,
    V2,
}

pub struct ClientEncryptArg {
	pub plaintext: String,
	pub lockdate: DateTime<FixedOffset>,
	pub hint: String,
}

pub struct ClientDecryptResultSuccess {
	pub plaintext: String,
	pub hint: String,
}

impl string::ToString for ClientDecryptResultSuccess {
	fn to_string(&self) -> String {
    	return self.plaintext
    		.clone();
	}
}

impl fmt::Debug for ClientDecryptResultSuccess {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	f.debug_struct("ClientDecryptResultSuccess")
    		.field("plaintext", &self.plaintext)
    		.field("hint", &self.hint)
    		.finish()
	}
}

pub struct ClientDecryptResultFailure {
	pub error_message: String,
	pub hint: String,
}

impl string::ToString for ClientDecryptResultFailure {
	fn to_string(&self) -> String {
    	return self.error_message
    		.clone();
	}
}

impl fmt::Debug for ClientDecryptResultFailure {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    	f.debug_struct("ClientDecryptResultFailure")
    		.field("error_message", &self.error_message)
    		.field("hint", &self.hint)
    		.finish()
	}
}

impl fmt::Display for ClientVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ClientVersion::V1 => write!(f, "1"),
			ClientVersion::V2 => write!(f, "2"),
		}        
    }
}

pub trait Client {
    fn encrypt(&self, args: &ClientEncryptArg) 
    	-> 
    	Result<String, String>;

    fn decrypt(
    	&self, 
    	ciphertext: &str
    	) 
    	->
    	Result<ClientDecryptResultSuccess, ClientDecryptResultFailure>;
    
    fn lockdate_from_snailcrypt_cipher(
    	&self, 
    	ciphertext: &str
    	)
    	->
    	Result<DateTime<FixedOffset>, String>;
        
    fn get_datetime_format(&self) -> &str;
    
	fn get_client_version(&self) -> ClientVersion;    
}
