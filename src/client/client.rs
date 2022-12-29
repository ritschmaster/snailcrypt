use std::fmt;

use chrono::{
    DateTime,
    FixedOffset,
};

#[derive(Debug, PartialEq, Eq)]
pub enum ClientVersion {
    V1,
}

impl fmt::Display for ClientVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ClientVersion::V1 => write!(f, "1")
		}        
    }
}

pub trait Client {
    fn encrypt(&self, plaintext: &str, lockdate: DateTime<FixedOffset>) -> Result<String, &'static str>;

    fn decrypt(&self, ciphertext: &str) -> Result<String, &'static str>;
        
    fn get_datetime_format(&self) -> &str;
    
	fn get_client_version(&self) -> ClientVersion;    
}
