use std::env;
use std::sync::Arc;

use reqwest::Client as HttpClient;

use klavier::config::Config;
use klavier::client::{Client, RequestResult};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    let test_config = Config::from_filepath(&args[1])?;

    let base_url: &String = &test_config.target.base_url;
    println!("Loaded config with base_url: {}", base_url);

    let http_client = HttpClient::builder()
    .pool_max_idle_per_host(100)     // Much higher for localhost
    .pool_idle_timeout(None)         // Keep connections alive
    .tcp_nodelay(true)               // Reduce local latency
    .build()?;

    let mut client: Client = Client::build_client(Arc::new(test_config), http_client)?;

    let mut idx = 0;
    while idx < 1000 {
        let result: RequestResult = client.run_next_scenario().await?;
        println!("Status: {} | Response: {} | Response Time: {}ms", &result.status, &result.body, &result.response_time);
        idx += 1;
    }

    Ok(())
}
