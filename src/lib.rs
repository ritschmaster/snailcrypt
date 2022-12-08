pub mod factory;
pub mod client;

// #[no_mangle]
// pub extern "C" fn add(left: usize, right: usize) -> usize {
//     left + right
// }

#[cfg(test)]
mod tests {
    use super::*;

    use chrono::{
        DateTime,
        FixedOffset,
    };

    use url::form_urlencoded;

    #[test]
    fn small_str() {
        let plaintext_orig: String = String::from("hello world");
        
		let factory: factory::ClientFactory = factory::ClientFactory::new();
        let client: Box<dyn client::Client> = factory.create();

        let lockdate: DateTime<FixedOffset> = DateTime::parse_from_str("2022-11-19T17:00:00+0100",
                                                                       client.get_datetime_format())
            .unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });

        let cipher: String = client.encrypt(plaintext_orig.as_str(), lockdate).unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });


        let plaintext: String = client.decrypt(cipher.as_str()).unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });

        assert_eq!(plaintext_orig.as_str(),
                   plaintext.as_str());
	}
	
	#[test]
	fn large_str() {
		let plaintext_orig: String = String::from("Nullam eu ante vel est convallis dignissim.  Fusce suscipit, wisi nec facilisis facilisis, est dui fermentum leo, 
quis tempor ligula erat quis odio.  Nunc porta vulputate tellus.  
Nunc rutrum turpis sed pede.  Sed bibendum.  Aliquam posuere.  
Nunc aliquet, augue nec adipiscing interdum, lacus tellus malesuada 
massa, quis varius mi purus non odio.  Pellentesque condimentum, 
magna ut suscipit hendrerit, ipsum augue ornare nulla, non 
luctus diam neque sit amet urna.  Curabitur vulputate vestibulum 
lorem.  Fusce sagittis, libero non molestie mollis, magna orci 
ultrices dolor, at vulputate neque nulla lacinia eros.  Sed id ligula
quis est convallis tempor.  Curabitur lacinia pulvinar nibh.  Nam a sapien.");
       		
		let factory: factory::ClientFactory = factory::ClientFactory::new();
        let client: Box<dyn client::Client> = factory.create();

        let lockdate: DateTime<FixedOffset> = DateTime::parse_from_str("2022-11-19T17:00:00+0100",
                                                                       client.get_datetime_format())
            .unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });

        let cipher: String = client.encrypt(plaintext_orig.as_str(), lockdate).unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });


        let plaintext: String = client.decrypt(cipher.as_str()).unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });

        assert_eq!(plaintext_orig.as_str(),
                   plaintext.as_str());
	}

	#[test]
	fn url() {
        let plaintext_orig: String = String::from("hello world");
        
		let factory: factory::ClientFactory = factory::ClientFactory::new();
        let client: Box<dyn client::Client> = factory.create();

        let lockdate: DateTime<FixedOffset> = DateTime::parse_from_str("2022-11-19T17:00:00+0100",
                                                                       client.get_datetime_format())
            .unwrap_or_else(|error| {
            panic!("Error: {:?}", error);
        });

        let cipher: String = client.encrypt(plaintext_orig.as_str(), lockdate).unwrap_or_else(|error| {
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
}
