use chrono::{
    DateTime,
    FixedOffset,
};

pub trait Client {
    fn encrypt(&self, plaintext: &str, lockdate: DateTime<FixedOffset>) -> Result<String, &'static str>;

    fn decrypt(&self, ciphertext: &str) -> Result<String, &'static str>;

    fn get_api_url(&self) -> &str;

    fn get_datetime_format(&self) -> &str;
}
