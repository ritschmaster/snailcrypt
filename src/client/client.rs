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
