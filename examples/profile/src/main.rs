use userapi::apis::configuration;
use userapi::apis::profile_api::{user_get_user_profile};
use futures::executor::block_on;
use userapi::apis::configuration::ApiKey;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Needs a token")
    }

    let api_key = ApiKey{
        prefix: None,
        key: String::from(&args[1]),
    };

    let mut config = configuration::Configuration::new();
    config.api_key = Some(api_key);

    let profile_result = user_get_user_profile(&config);

    match block_on(profile_result) {
        Ok(resp) => {
            println!("Success! Profile = {:?}", resp);
        },
        Err(error) => {
            println!("error: {:?}", error);
            panic!("Query failed!")
        }
    }
}
