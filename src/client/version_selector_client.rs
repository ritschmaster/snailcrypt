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

use crate::client::Client;

use std::rc::Rc;

use chrono::{
    DateTime,
    FixedOffset,
};
use serde_json::Value;

#[allow(unused)]
pub struct VersionSelectorClient {
}

/**
 * This object implements an automatic switch between the available versions of the clients. Depending on the input parameters for the encryption and decryption it will automatically choose the applicable client version and uses its implementation for the requested operation.
 *
 * NOT IMPLEMENTED YET
 */
impl VersionSelectorClient {
    #[allow(unused)]
    pub fn new(api_url: &str) -> VersionSelectorClient {
        return VersionSelectorClient {  };
    }
}

impl Client for VersionSelectorClient {
    fn encrypt(&self, plaintext: &str, lockdate: DateTime<FixedOffset>) -> Result<String, &'static str> {
        Ok(String::from("hello world"))
    }

    fn decrypt(&self, ciphertext: &str) -> Result<String, &'static str> {
        Ok(String::from("hello world"))
    }

    fn get_api_url(&self) -> &str {
        return "hello world";
    }


    fn get_datetime_format(&self) -> &str {
        return "%Y-%m-%dT%H:%M:%S%z";
    }
}
