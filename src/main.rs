use std::sync::Arc;

use reqwest::Client as HttpClient;

use klavier::config::Config;
use klavier::client::{Client, RequestResult};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let test_config = Config::from_filepath("profiles/example.toml")?;

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
        println!("Status: {} | Response: {}", &result.status, &result.body);
        idx += 1;
    }

    Ok(())
}
