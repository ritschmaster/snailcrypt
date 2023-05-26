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

pub use crate::{
    client::{
        Client,
        ClientVersion,
        ClientEncryptArg,
        ClientDecryptResultSuccess,
        ClientDecryptResultFailure,
        V1Client,
        V2Client,
    },
    util::Analyzer,
};

use std::rc::Rc;

use chrono::{
    DateTime,
    FixedOffset,
};

#[allow(unused)]
pub struct VersionSelectorClient {
    analyzer: Rc<dyn Analyzer>,
    v1_client: Rc<dyn Client>,
    v2_client: Rc<dyn Client>,
}

/// This object implements an automatic switch between the available versions of the clients. Depending on the input parameters for the encryption and decryption it will automatically choose the applicable client version and uses its implementation for the requested operation.
///
/// NOT IMPLEMENTED YET
impl VersionSelectorClient {
    #[allow(unused)]
    pub fn new(analyzer: Rc<dyn Analyzer>, v1_client: Rc<dyn Client>, v2_client: Rc<dyn Client>) -> VersionSelectorClient {
        return VersionSelectorClient {
            analyzer,
            v1_client,
            v2_client,
        };
    }
    
    pub fn get_analyzer(&self) -> &Rc<dyn Analyzer> {
        return &self.analyzer;
    }
}

impl Client for VersionSelectorClient {
    fn encrypt(&self, args: &ClientEncryptArg)
        ->
        Result<String, String> {
        if args.hint.len() > 0 {
            self.v2_client.encrypt(args)
        } else {
            self.v1_client.encrypt(args)
        }
    }

    fn decrypt(
        &self,
        ciphertext: &str
        )
        ->
        Result<ClientDecryptResultSuccess, ClientDecryptResultFailure> {
        let result = self.get_analyzer()
            .get_version(ciphertext);
        
        if result.is_err() {
            return Err(ClientDecryptResultFailure {
                error_message: result.unwrap_err(),
                hint: String::from(""),
            });
        }
        
        let version = result.unwrap();
        
        match version {
            ClientVersion::V1 => return self.v1_client.decrypt(ciphertext),
            ClientVersion::V2 => return self.v2_client.decrypt(ciphertext),
        }
    }
    
    fn lockdate_from_snailcrypt_cipher(
        &self,
        ciphertext: &str
        )
        ->
        Result<DateTime<FixedOffset>, String> {
        Ok(self.v1_client.lockdate_from_snailcrypt_cipher(ciphertext).unwrap())
    }

    fn get_datetime_format(&self) -> &str {
        return self.v1_client.get_datetime_format();
    }
    
    fn get_client_version(&self) -> ClientVersion {
        return self.v1_client.get_client_version();
    }
}
