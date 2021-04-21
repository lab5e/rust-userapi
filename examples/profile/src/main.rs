use std::env;
use futures::executor::block_on;
use userapi::apis::configuration;
use userapi::apis::configuration::ApiKey;
use userapi::apis::profile_api::{get_user_profile};

#[tokio::main]
async fn main() {
    // Get the token from the command line.
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Needs a token")
    }

    // Create the api key struct for the token
    let api_key = ApiKey{
        prefix: None,
        key: String::from(&args[1]),
    };

    // Create configuration and add the API key struct to it.
    let mut config = configuration::Configuration::new();
    config.api_key = Some(api_key);

    // Finally -- get the profile
    let profile_result = get_user_profile(&config);

    // ..and print the result
    match block_on(profile_result) {
        Ok(resp) => {
            println!("Profile");
            println!("=======");
            println!("User ID: {}", resp.user_id.unwrap_or_default());
            println!("Name:    {}", resp.name.unwrap_or_default());
            println!("Email:   {}", resp.email.unwrap_or_default());
            println!("Avatar:  {}", resp.avatar_url.unwrap_or_default());
        },
        Err(error) => {
            println!("error: {:?}", error);
            panic!("Query failed!")
        }
    }
}
