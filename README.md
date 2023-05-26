# snailcrypt

Library to access api.snailcrypt.com

## Example usages

### Encrypting a string

To encrypt a string for a given date you can place the following code in your application:

```rust
use snailcrypt::{
    client,
    config,
    factory,
    util,
};

use std::rc::Rc;

use chrono::{
    DateTime,
    FixedOffset,
};

/**
 * Setup main input data
 */
let plaintext = String::from("hello world");
let hint = String::from("This is a test message");

/**
 * Produce a client
 */
let analyzer_factory: factory::AnalyzerFactory = factory::AnalyzerFactory::new();
let analyzer: Rc<dyn util::Analyzer> = analyzer_factory.create();

let config_factory: factory::ConfigFactory = factory::ConfigFactory::new();
let config: Rc<dyn config::Config> = config_factory.create();

let client_factory: factory::ClientFactory = factory::ClientFactory::new(Rc::clone(&analyzer),
                            Rc::clone(&config));
let client: Rc<dyn client::Client> = client_factory.create();


let lockdate: DateTime<FixedOffset> = DateTime::parse_from_str("2022-11-19T17:00:00+0100",
    client.get_datetime_format())
    .unwrap_or_else(|error| {
        panic!("Error: {:?}", error);
    });

/**
 * Perform encryption using the client
 */
let cipher: String = client.encrypt(&client::ClientEncryptArg {
        plaintext,
        lockdate,
        hint,
    })
    .unwrap_or_else(|error| {
        panic!("Error: {:?}", error);
    });
```

### Decrypting a string

To decrypt a snailcrypt string you can place the following code in your application:

```rust
use snailcrypt::{
    client,
    config,
    factory,
    util,
};

use std::rc::Rc;

use chrono::{
    DateTime,
    FixedOffset,
};

/**
 * Setup main input data
 */
let cipher: String = String::from("1:MjAyMi0xMS0xOVQxNzowMDowMCswMTAw:bVoPtqSST34ojbXQHEdTfQuvCgI7Ed/SsBLSNczVhoCSmMcpJNv3/rAGomn+hNJihmzOu7RQXDTNEnkewV4TXrMGuWqvfmCIAPTTQnuUkqLimuL8WD2Nu8LY2LaPMf3G1Q9JiRb+dd7lmboppgOd9ssPciAXTiI0NkJ4SawBW/PVWOuEFAWDs2MBkPT6oxbJrNha5L0lHDpgHMTP9HsdVf3gh9GiKuwQFtaZ3WXKTKUnOPALz3QkcLOspFHP+UuOUuZn4OrkxpWGbTFqS00NROwT4a5V0vbY/Ag+RYJtd9Pk3UsTT4QNUSj1vQ81X27tC6+B8gXxaVGWRynIgYn5wQ==");

/**
 * Produce a client
 */
let analyzer_factory: factory::AnalyzerFactory = factory::AnalyzerFactory::new();
let analyzer: Rc<dyn util::Analyzer> = analyzer_factory.create();

let config_factory: factory::ConfigFactory = factory::ConfigFactory::new();
let config: Rc<dyn config::Config> = config_factory.create();

let client_factory: factory::ClientFactory = factory::ClientFactory::new(Rc::clone(&analyzer),
                            Rc::clone(&config));
let client: Rc<dyn client::Client> = client_factory.create();

/**
 * Perform decryption using the client
 */
let result = client
    .decrypt(cipher.as_str())
    .unwrap_or_else(|error| {
        panic!("Error: {:?}", error.error_message);
    });   

println!("{}", result.plaintext.as_str());     
println!("{}", result.hint.as_str());
```
