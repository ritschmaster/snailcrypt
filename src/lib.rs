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

pub mod client;
pub mod config;
pub mod factory;
pub mod util;

// #[no_mangle]
// pub extern "C" fn add(left: usize, right: usize) -> usize {
//     left + right
// }

#[cfg(test)]
mod tests {
    use super::*;
    
    use std::rc::Rc;

    use chrono::{
        DateTime,
        FixedOffset,
    };

    use url::form_urlencoded;
    
    #[test]
    fn version_v1_parse_ok() {
		let analyzer_factory: factory::AnalyzerFactory = factory::AnalyzerFactory::new();
		let analyzer = analyzer_factory.create();
	
		/* Arbitary text */	
		assert!(analyzer.get_version("1:asdf:asdf")
						   .unwrap_or_else(|error| { 
								panic!("Error: {:?}", error); 
							})
				==
				client::ClientVersion::V1);
				
		/* Actual cipher text */	
		assert!(analyzer.get_version("1:MjAyMi0xMS0xOVQxNzowMDowMCswMTAw:jbTjg8y9U5g95BP/LKQHbE2pSBDYFnILpgFbqFfqXbdMYUGUh3v1R040d+eHZuYzOe55qKf8Q16J8zvawKmMejhlVGTOhLHobnJvtL08S184v9/HxGL1A1ZrtgoAiuxd7DZLLxAOQSzJoBlRG2jz9AhcCQI5pXn1EujvMICv2dusnmrjuzxPRnu2NtaXJNpzEycGSwTxoXuxWOb93YXaJlVOcS7mMjSQG5tLBA84AYFoeqJcERITSzsRcckMU0uOEWLm66OtiLrDRgmRo/0xSUIn+kocjI7RExl1FgeeqppDuR1C9CgCrIicSbvsiqn6zlrf1wyz+lMw0sUGOCU3xQ==")
						   .unwrap_or_else(|error| { 
								panic!("Error: {:?}", error); 
							})
				==
				client::ClientVersion::V1);		
	}
	
    #[test]
    fn version_v1_parse_nok() {
		let mut error_thrown: bool;
	
		let analyzer_factory: factory::AnalyzerFactory = factory::AnalyzerFactory::new();
		let analyzer = analyzer_factory.create();
	
		/* Arbitary text */	
		error_thrown = false;
		analyzer.get_version(":asdf:asdf")
			    .unwrap_or_else(|error| {					
					assert_eq!(error.to_string().as_str(),
							   "Unknown client version: ");
							 
					error_thrown = true;								
					return client::ClientVersion::V1;
				});
		assert_eq!(error_thrown, true);
				
		/* Actual cipher text */			
		error_thrown = false;
		analyzer.get_version("MjAyMi0xMS0xOVQxNzowMDowMCswMTAw:jbTjg8y9U5g95BP/LKQHbE2pSBDYFnILpgFbqFfqXbdMYUGUh3v1R040d+eHZuYzOe55qKf8Q16J8zvawKmMejhlVGTOhLHobnJvtL08S184v9/HxGL1A1ZrtgoAiuxd7DZLLxAOQSzJoBlRG2jz9AhcCQI5pXn1EujvMICv2dusnmrjuzxPRnu2NtaXJNpzEycGSwTxoXuxWOb93YXaJlVOcS7mMjSQG5tLBA84AYFoeqJcERITSzsRcckMU0uOEWLm66OtiLrDRgmRo/0xSUIn+kocjI7RExl1FgeeqppDuR1C9CgCrIicSbvsiqn6zlrf1wyz+lMw0sUGOCU3xQ==")				
				.unwrap_or_else(|error| { 
					assert_eq!(error.to_string().as_str(),
							   "Unknown client version: MjAyMi0xMS0xOVQxNzowMDowMCswMTAw");
							 
					error_thrown = true;
					
					return client::ClientVersion::V1;
				});		
		assert_eq!(error_thrown, true);
	}

    #[test]
    fn encrypt_v1_small_str() {
        let plaintext_orig: String = String::from("hello world");

 		let analyzer_factory: factory::AnalyzerFactory = factory::AnalyzerFactory::new();
        let analyzer: Rc<dyn util::Analyzer> = analyzer_factory.create();
   
		let config_factory: factory::ConfigFactory = factory::ConfigFactory::new();
        let config: Rc<dyn config::Config> = config_factory.create();
  
		let client_factory: factory::ClientFactory = factory::ClientFactory::new(Rc::clone(&analyzer),
                                        Rc::clone(&config));
        let client: Rc<dyn client::Client> = client_factory.create(client::ClientVersion::V1);

        let lockdate: DateTime<FixedOffset> = DateTime::parse_from_str("2022-11-19T17:00:00+0100",
                                                                       client.get_datetime_format())
            .unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });

        let cipher: String = client.encrypt(&client::ClientEncryptArg {
        	plaintext: plaintext_orig.clone(),
        	lockdate,
        	hint: String::from(""),
        }).unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });


        let result_success = client
        	.decrypt(cipher
        		.as_str())
       		.unwrap_or_else(|error| {
           		panic!("Error: {:?}", error.error_message);
        	});

		assert_eq!(plaintext_orig.as_str(),
                   result_success.plaintext.as_str());
                   
        assert_eq!("",
        		   result_success.hint.as_str());
	}

	#[test]
	fn encrypt_v1_large_str() {
		let plaintext_orig: String = 
			String::from("Nullam eu ante vel est convallis dignissim.  Fusce suscipit, wisi nec facilisis facilisis, est dui fermentum leo, 
quis tempor ligula erat quis odio.  Nunc porta vulputate tellus.  
Nunc rutrum turpis sed pede.  Sed bibendum.  Aliquam posuere.  
Nunc aliquet, augue nec adipiscing interdum, lacus tellus malesuada 
massa, quis varius mi purus non odio.  Pellentesque condimentum, 
magna ut suscipit hendrerit, ipsum augue ornare nulla, non 
luctus diam neque sit amet urna.  Curabitur vulputate vestibulum 
lorem.  Fusce sagittis, libero non molestie mollis, magna orci 
ultrices dolor, at vulputate neque nulla lacinia eros.  Sed id ligula
quis est convallis tempor.  Curabitur lacinia pulvinar nibh.  Nam a sapien.");
       	
       	let analyzer_factory: factory::AnalyzerFactory = factory::AnalyzerFactory::new();
        let analyzer: Rc<dyn util::Analyzer> = analyzer_factory.create();
        
		let config_factory: factory::ConfigFactory = factory::ConfigFactory::new();
        let config: Rc<dyn config::Config> = config_factory.create();
          
		let client_factory: factory::ClientFactory = factory::ClientFactory::new(Rc::clone(&analyzer),
                                        Rc::clone(&config));
        let client: Rc<dyn client::Client> = client_factory.create(client::ClientVersion::V1);

        let lockdate: DateTime<FixedOffset> = DateTime::parse_from_str("2022-11-19T17:00:00+0100",
                                                                       client.get_datetime_format())
            .unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });

        let cipher: String = client.encrypt(&client::ClientEncryptArg{
        	plaintext: plaintext_orig.clone(),
        	lockdate,
        	hint: String::from(""),
		}).unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });


        let result_success = client
        	.decrypt(cipher
       		.as_str())
       		.unwrap_or_else(|error| {
            	panic!("Error: {:?}", error.error_message);
	        });
        
		assert_eq!(plaintext_orig.as_str(),
                   result_success.plaintext.as_str());
                   
        assert_eq!("",
        		   result_success.hint.as_str());
	}

	#[test]
	fn encrypt_v1_url() {
        let plaintext_orig: String = String::from("hello world");

		let analyzer_factory: factory::AnalyzerFactory = factory::AnalyzerFactory::new();
        let analyzer: Rc<dyn util::Analyzer> = analyzer_factory.create();
        
		let config_factory: factory::ConfigFactory = factory::ConfigFactory::new();
        let config: Rc<dyn config::Config> = config_factory.create();
                
		let client_factory: factory::ClientFactory = factory::ClientFactory::new(Rc::clone(&analyzer),
                                        Rc::clone(&config));
        let client: Rc<dyn client::Client> = client_factory.create(client::ClientVersion::V1);

        let lockdate: DateTime<FixedOffset> = DateTime::parse_from_str("2022-11-19T17:00:00+0100",
                                                                       client.get_datetime_format())
            .unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });

        let cipher: String = client.encrypt(&client::ClientEncryptArg{
        	plaintext: plaintext_orig,
        	lockdate,
        	hint: String::from(""),
		}).unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });
		
        let mut cipher_url: String = String::from("https://webapp.snailcrypt.com/timer.php?");
        cipher_url.push_str(form_urlencoded::Serializer::new(String::new())
                            .append_pair("c",
                                         cipher.as_str())
                            .finish()
                            .as_str());

        // cipher_url.push_str("c=");
        // cipher_url.push_str(cipher.as_str());

//        assert_eq!("LPX/bp1RFJR7BOpJVWa8ewFQPjF1k5bxU+OtiOg/3CgsVpcyF6m32nLIkhptrB0QcS+XiV/EpgBXLDOFL8n/6n/srdidy1f8s+oGn05U7zUkVTNPeSTV5RlAL+MvJ3dp+rhfoSIlkauY5Iioo+u/Vf/kab3K5hlRhawpG2aw4990TfDQmyvNZVYfy+2yuxk1M+bkZOK7PNZ0ntQPPvHkY4XIZbjzQkEIDm8fIKnqf2Nji1GU2bkB0YUykKE4/9D20Ul7RLnvhzyPckrePygyVCMMwOUH41yzVAAmKpkE0vmd8sbM7kVHGxer9PRFrXjTUoKYzz4YcYspbcwjhXUuS92hMqXr6w8zIt80zK5AvLrAKcjlyCwU7juWE0WBuawkfddi0LAqvcOYhF0cPQY/KhZa/jwDU2+QS3R1fCOlT2aYumsOL3m6oat5oKPctPB5EULXgx5C9Jsn6OYMz0Wmxj+RSvgOthCVOmfcG0luaU98l582k8XjXc8m08HyqmsPpY19gxrL/CJS9w7syKSk8SJLIXChhken1cLzjCw8YMYXtaJw4ChBypiso+yQpOZFgOSHpT6Iuo38N93wK0C0mzL7EXC5w+kg3n/J95tqPu+V7EI1ArgiIDC2Cg1mKaHSkbhFbaUJrooZOy2T+q4ouFuJQXu+oWovo6wULz3iIIo=",
//                   cipher_url.as_str());
    }

	#[test]    
    fn version_v2_parse_ok() {
		let analyzer_factory: factory::AnalyzerFactory = factory::AnalyzerFactory::new();
		let analyzer = analyzer_factory.create();
	
		/* Arbitary text */	
		assert!(analyzer.get_version("2:asdf:asdf:asdf")
						   .unwrap_or_else(|error| { 
								panic!("Error: {:?}", error); 
							})
				==
				client::ClientVersion::V2);
				
		/* Actual cipher text */	
		assert!(analyzer.get_version("2:MjAyMi0xMS0xOVQxNzowMDowMCswMTAw:jbTjg8y9U5g95BP/LKQHbE2pSBDYFnILpgFbqFfqXbdMYUGUh3v1R040d+eHZuYzOe55qKf8Q16J8zvawKmMejhlVGTOhLHobnJvtL08S184v9/HxGL1A1ZrtgoAiuxd7DZLLxAOQSzJoBlRG2jz9AhcCQI5pXn1EujvMICv2dusnmrjuzxPRnu2NtaXJNpzEycGSwTxoXuxWOb93YXaJlVOcS7mMjSQG5tLBA84AYFoeqJcERITSzsRcckMU0uOEWLm66OtiLrDRgmRo/0xSUIn+kocjI7RExl1FgeeqppDuR1C9CgCrIicSbvsiqn6zlrf1wyz+lMw0sUGOCU3xQ==:asdf")
						   .unwrap_or_else(|error| { 
								panic!("Error: {:?}", error); 
							})
				==
				client::ClientVersion::V2);		
	}
	
    #[test]
    fn version_v2_parse_nok() {
		let mut error_thrown: bool;
	
		let analyzer_factory: factory::AnalyzerFactory = factory::AnalyzerFactory::new();
		let analyzer = analyzer_factory.create();
	
		/* Arbitary text */	
		error_thrown = false;
		analyzer.get_version(":asdf:asdf:asdf")
			    .unwrap_or_else(|error| {					
					assert_eq!(error.to_string().as_str(),
							   "Unknown client version: ");
							 
					error_thrown = true;								
					return client::ClientVersion::V2;
				});
		assert_eq!(error_thrown, true);
				
		/* Actual cipher text */			
		error_thrown = false;
		analyzer.get_version("MjAyMi0xMS0xOVQxNzowMDowMCswMTAw:jbTjg8y9U5g95BP/LKQHbE2pSBDYFnILpgFbqFfqXbdMYUGUh3v1R040d+eHZuYzOe55qKf8Q16J8zvawKmMejhlVGTOhLHobnJvtL08S184v9/HxGL1A1ZrtgoAiuxd7DZLLxAOQSzJoBlRG2jz9AhcCQI5pXn1EujvMICv2dusnmrjuzxPRnu2NtaXJNpzEycGSwTxoXuxWOb93YXaJlVOcS7mMjSQG5tLBA84AYFoeqJcERITSzsRcckMU0uOEWLm66OtiLrDRgmRo/0xSUIn+kocjI7RExl1FgeeqppDuR1C9CgCrIicSbvsiqn6zlrf1wyz+lMw0sUGOCU3xQ==:asdf")				
				.unwrap_or_else(|error| { 
					assert_eq!(error.to_string().as_str(),
							   "Unknown client version: MjAyMi0xMS0xOVQxNzowMDowMCswMTAw");
							 
					error_thrown = true;
					
					return client::ClientVersion::V2;
				});		
		assert_eq!(error_thrown, true);
	}

    
    #[test]
    fn encrypt_v2_small_str() {
        let plaintext_orig = String::from("hello world");
        let hint_orig = String::from("This is a test message");

 		let analyzer_factory: factory::AnalyzerFactory = factory::AnalyzerFactory::new();
        let analyzer: Rc<dyn util::Analyzer> = analyzer_factory.create();
   
		let config_factory: factory::ConfigFactory = factory::ConfigFactory::new();
        let config: Rc<dyn config::Config> = config_factory.create();
        
		let client_factory: factory::ClientFactory = factory::ClientFactory::new(Rc::clone(&analyzer),
                                        Rc::clone(&config));
        let client: Rc<dyn client::Client> = client_factory.create(client::ClientVersion::V2);

        let lockdate: DateTime<FixedOffset> = DateTime::parse_from_str("2022-11-19T17:00:00+0100",
                                                                       client.get_datetime_format())
            .unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });

        let cipher: String = client.encrypt(&client::ClientEncryptArg {
        	plaintext: plaintext_orig.clone(),
        	lockdate,
        	hint: hint_orig.clone(),
        }).unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });


        let result_success = client
        	.decrypt(cipher
	       		.as_str())
	       	.unwrap_or_else(|error| {
            	panic!("Error: {:?}", error.error_message);
	       	});

        assert_eq!(plaintext_orig.as_str(),
                   result_success.plaintext.as_str());
                   
        assert_eq!(hint_orig.as_str(),
        		   result_success.hint.as_str());
	}
	
	#[test]
    fn encrypt_v2_larg_str() {
		let plaintext_orig: String = 
			String::from("Nullam eu ante vel est convallis dignissim.  Fusce suscipit, wisi nec facilisis facilisis, est dui fermentum leo, 
quis tempor ligula erat quis odio.  Nunc porta vulputate tellus.  
Nunc rutrum turpis sed pede.  Sed bibendum.  Aliquam posuere.  
Nunc aliquet, augue nec adipiscing interdum, lacus tellus malesuada 
massa, quis varius mi purus non odio.  Pellentesque condimentum, 
magna ut suscipit hendrerit, ipsum augue ornare nulla, non 
luctus diam neque sit amet urna.  Curabitur vulputate vestibulum 
lorem.  Fusce sagittis, libero non molestie mollis, magna orci 
ultrices dolor, at vulputate neque nulla lacinia eros.  Sed id ligula
quis est convallis tempor.  Curabitur lacinia pulvinar nibh.  Nam a sapien.");
        let hint_orig = 
        	String::from("Nullam eu ante vel est convallis dignissim.  Fusce suscipit, wisi nec facilisis facilisis, est dui fermentum leo, 
quis tempor ligula erat quis odio.  Nunc porta vulputate tellus.  
Nunc rutrum turpis sed pede.  Sed bibendum.  Aliquam posuere.  
Nunc aliquet, augue nec adipiscing interdum, lacus tellus malesuada 
massa, quis varius mi purus non odio.  Pellentesque condimentum, 
magna ut suscipit hendrerit, ipsum augue ornare nulla, non 
luctus diam neque sit amet urna.  Curabitur vulputate vestibulum 
lorem.  Fusce sagittis, libero non molestie mollis, magna orci 
ultrices dolor, at vulputate neque nulla lacinia eros.  Sed id ligula
quis est convallis tempor.  Curabitur lacinia pulvinar nibh.  Nam a sapien.");

 		let analyzer_factory: factory::AnalyzerFactory = factory::AnalyzerFactory::new();
        let analyzer: Rc<dyn util::Analyzer> = analyzer_factory.create();
   
		let config_factory: factory::ConfigFactory = factory::ConfigFactory::new();
        let config: Rc<dyn config::Config> = config_factory.create();
        
		let client_factory: factory::ClientFactory = factory::ClientFactory::new(Rc::clone(&analyzer),
                                        Rc::clone(&config));
        let client: Rc<dyn client::Client> = client_factory.create(client::ClientVersion::V2);

        let lockdate: DateTime<FixedOffset> = DateTime::parse_from_str("2022-11-19T17:00:00+0100",
                                                                       client.get_datetime_format())
            .unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });

        let cipher: String = client.encrypt(&client::ClientEncryptArg {
        	plaintext: plaintext_orig.clone(),
        	lockdate,
        	hint: hint_orig.clone(),
        }).unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });


        let result_success = client
        	.decrypt(cipher
	       		.as_str())
	       	.unwrap_or_else(|error| {
            	panic!("Error: {:?}", error.error_message);
	       	});

        assert_eq!(plaintext_orig.as_str(),
                   result_success.plaintext.as_str());
                   
        assert_eq!(hint_orig.as_str(),
        		   result_success.hint.as_str());
	}
 
    #[test]
    fn encrypt_vguess_small_str() {
        let plaintext_orig: String = String::from("hello world");

        let analyzer_factory: factory::AnalyzerFactory = factory::AnalyzerFactory::new();
        let analyzer: Rc<dyn util::Analyzer> = analyzer_factory.create();
   
        let config_factory: factory::ConfigFactory = factory::ConfigFactory::new();
        let config: Rc<dyn config::Config> = config_factory.create();
  
        let client_factory: factory::ClientFactory = factory::ClientFactory::new(Rc::clone(&analyzer),
                                        Rc::clone(&config));
        let client: Rc<dyn client::Client> = client_factory.create(client::ClientVersion::V1);

        let lockdate: DateTime<FixedOffset> = DateTime::parse_from_str("2022-11-19T17:00:00+0100",
                                                                       client.get_datetime_format())
            .unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });

        let cipher: String = client.encrypt(&client::ClientEncryptArg {
            plaintext: plaintext_orig.clone(),
            lockdate,
            hint: String::from(""),
        }).unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });

        let client_decryption: Rc<dyn client::Client> = client_factory.create(analyzer
            .get_version(cipher
                .as_str())
            .unwrap_or_else(|error| {
                panic!("Error: {:?}", error);
            }));
        let result_success = client_decryption
            .decrypt(cipher
                .as_str())
               .unwrap_or_else(|error| {
                   panic!("Error: {:?}", error.error_message);
            });

        assert_eq!(plaintext_orig.as_str(),
                   result_success.plaintext.as_str());
                   
        assert_eq!("",
                   result_success.hint.as_str());
    }

    #[test]
    fn encrypt_vguess_large_str() {
        let plaintext_orig: String =
            String::from("Nullam eu ante vel est convallis dignissim.  Fusce suscipit, wisi nec facilisis facilisis, est dui fermentum leo,
quis tempor ligula erat quis odio.  Nunc porta vulputate tellus.
Nunc rutrum turpis sed pede.  Sed bibendum.  Aliquam posuere.
Nunc aliquet, augue nec adipiscing interdum, lacus tellus malesuada
massa, quis varius mi purus non odio.  Pellentesque condimentum,
magna ut suscipit hendrerit, ipsum augue ornare nulla, non
luctus diam neque sit amet urna.  Curabitur vulputate vestibulum
lorem.  Fusce sagittis, libero non molestie mollis, magna orci
ultrices dolor, at vulputate neque nulla lacinia eros.  Sed id ligula
quis est convallis tempor.  Curabitur lacinia pulvinar nibh.  Nam a sapien.");
           
           let analyzer_factory: factory::AnalyzerFactory = factory::AnalyzerFactory::new();
        let analyzer: Rc<dyn util::Analyzer> = analyzer_factory.create();
        
        let config_factory: factory::ConfigFactory = factory::ConfigFactory::new();
        let config: Rc<dyn config::Config> = config_factory.create();
          
        let client_factory: factory::ClientFactory = factory::ClientFactory::new(Rc::clone(&analyzer),
                                        Rc::clone(&config));
        let client: Rc<dyn client::Client> = client_factory.create(client::ClientVersion::V1);
        
        let lockdate: DateTime<FixedOffset> = DateTime::parse_from_str("2022-11-19T17:00:00+0100",
                                                                       client.get_datetime_format())
            .unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });

        let cipher: String = client.encrypt(&client::ClientEncryptArg{
            plaintext: plaintext_orig.clone(),
            lockdate,
            hint: String::from(""),
        }).unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });

        let client_decryption: Rc<dyn client::Client> = client_factory.create(analyzer
            .get_version(cipher
                .as_str())
            .unwrap_or_else(|error| {
                panic!("Error {:?}", error);
            }));
        let result_success = client_decryption
            .decrypt(cipher
               .as_str())
               .unwrap_or_else(|error| {
                panic!("Error: {:?}", error.error_message);
            });
        
        assert_eq!(plaintext_orig.as_str(),
                   result_success.plaintext.as_str());
                   
        assert_eq!("",
                   result_success.hint.as_str());
    }
/*
    #[test]
    fn encrypt_version_selector_small_str() {
        let plaintext_orig: String = String::from("hello world");

        let analyzer_factory: factory::AnalyzerFactory = factory::AnalyzerFactory::new();
        let analyzer: Rc<dyn util::Analyzer> = analyzer_factory.create();
   
        let config_factory: factory::ConfigFactory = factory::ConfigFactory::new();
        let config: Rc<dyn config::Config> = config_factory.create();
  
        let client_factory: factory::ClientFactory = factory::ClientFactory::new(Rc::clone(&analyzer),
                                        Rc::clone(&config));
        let client: Rc<dyn client::Client> = client_factory.create(client::ClientVersion::V1);

        let lockdate: DateTime<FixedOffset> = DateTime::parse_from_str("2022-11-19T17:00:00+0100",
                                                                       client.get_datetime_format())
            .unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });

        let cipher: String = client.encrypt(&client::ClientEncryptArg {
            plaintext: plaintext_orig.clone(),
            lockdate,
            hint: String::from(""),
        }).unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });

        let client_decryption: Rc<dyn client::Client> = client_factory.create(analyzer
            .get_version(cipher
                .as_str())
            .unwrap_or_else(|error| {
                panic!("Error: {:?}", error);
            }));
        let result_success = client_decryption
            .decrypt(cipher
                .as_str())
               .unwrap_or_else(|error| {
                   panic!("Error: {:?}", error.error_message);
            });

        assert_eq!(plaintext_orig.as_str(),
                   result_success.plaintext.as_str());
                   
        assert_eq!("",
                   result_success.hint.as_str());
    }

    #[test]
    fn encrypt_version_selector_large_str() {
        let plaintext_orig: String =
            String::from("Nullam eu ante vel est convallis dignissim.  Fusce suscipit, wisi nec facilisis facilisis, est dui fermentum leo,
quis tempor ligula erat quis odio.  Nunc porta vulputate tellus.
Nunc rutrum turpis sed pede.  Sed bibendum.  Aliquam posuere.
Nunc aliquet, augue nec adipiscing interdum, lacus tellus malesuada
massa, quis varius mi purus non odio.  Pellentesque condimentum,
magna ut suscipit hendrerit, ipsum augue ornare nulla, non
luctus diam neque sit amet urna.  Curabitur vulputate vestibulum
lorem.  Fusce sagittis, libero non molestie mollis, magna orci
ultrices dolor, at vulputate neque nulla lacinia eros.  Sed id ligula
quis est convallis tempor.  Curabitur lacinia pulvinar nibh.  Nam a sapien.");
           
           let analyzer_factory: factory::AnalyzerFactory = factory::AnalyzerFactory::new();
        let analyzer: Rc<dyn util::Analyzer> = analyzer_factory.create();
        
        let config_factory: factory::ConfigFactory = factory::ConfigFactory::new();
        let config: Rc<dyn config::Config> = config_factory.create();
          
        let client_factory: factory::ClientFactory = factory::ClientFactory::new(Rc::clone(&analyzer),
                                        Rc::clone(&config));
        let client: Rc<dyn client::Client> = client_factory.create(client::ClientVersion::V1);
        
        let lockdate: DateTime<FixedOffset> = DateTime::parse_from_str("2022-11-19T17:00:00+0100",
                                                                       client.get_datetime_format())
            .unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });

        let cipher: String = client.encrypt(&client::ClientEncryptArg{
            plaintext: plaintext_orig.clone(),
            lockdate,
            hint: String::from(""),
        }).unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });

        let client_decryption: Rc<dyn client::Client> = client_factory.create(analyzer
            .get_version(cipher
                .as_str())
            .unwrap_or_else(|error| {
                panic!("Error {:?}", error);
            }));
        let result_success = client_decryption
            .decrypt(cipher
               .as_str())
               .unwrap_or_else(|error| {
                panic!("Error: {:?}", error.error_message);
            });
        
        assert_eq!(plaintext_orig.as_str(),
                   result_success.plaintext.as_str());
                   
        assert_eq!("",
                   result_success.hint.as_str());
    }
*/
}
