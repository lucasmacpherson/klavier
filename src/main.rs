use std::env;
use std::sync::Arc;

use anyhow::{bail, Error};
use reqwest::Client as HttpClient;

use klavier::config::Config;
use klavier::client::Client;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("Please provide a profile path (e.g. profiles/example.toml)")
    }

    let test_config = Config::from_filepath(&args[1])?;

    let base_url: &String = &test_config.target.base_url;
    println!("Loaded config with base_url: {}", base_url);

    
    let mut client: Client = Client::build_client(Arc::new(test_config), http_client)?;
    let results = client.run().await?;

    for result in results {
        println!("Status: {} | Response: {} | Response Time: {}ms", &result.status, &result.body, &result.response_time);
    }

    Ok(())
}
