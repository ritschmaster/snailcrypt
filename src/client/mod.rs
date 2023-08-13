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

mod client;
mod v1_client;
mod v2_client;
mod v3_client;
mod version_selector_client;

pub use client::ClientVersion;
pub use client::ClientEncryptArg;
pub use client::ClientDecryptResultSuccess;
pub use client::ClientDecryptResultFailure;
pub use client::Client;
pub use v1_client::V1Client;
pub use v2_client::V2Client;
pub use v3_client::V3Client;
pub use version_selector_client::VersionSelectorClient;
