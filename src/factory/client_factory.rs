pub use crate::client::{ Client, DefaultClient };

#[allow(unused)]
pub struct ClientFactory {
}

impl ClientFactory {
    #[allow(unused)]
    pub fn new() -> ClientFactory {
        return ClientFactory { };
    }

    pub fn create(&self) -> Box<dyn Client> {
        return Box::new(DefaultClient::new("https://api.snailcrypt.com"));
    }
}
