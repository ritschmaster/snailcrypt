use crate::client::ClientVersion;

pub trait Analyzer {
    fn get_version(&self, ciphertext: &str) -> Result<ClientVersion, String>;
            
    fn str_to_version(&self, client_version: &str) -> Result<ClientVersion, String>;
}
