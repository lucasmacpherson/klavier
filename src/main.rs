use std::error::Error;

use klavier::config::Config;
use klavier::client::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let test_config = Config::from_filepath("profiles/example.toml")?;

    let base_url: &String = &test_config.target.base_url;
    println!("Loaded config with base_url: {}", base_url);

    let mut idx = 0;
    while idx < 10 {
        let &scenario = &test_config.get_next_scenario();
        println!("Scenario selected: {}", scenario.path);
        idx += 1;
    }

    let client = HttpClient::builder()
    .pool_max_idle_per_host(100)     // Much higher for localhost
    .pool_idle_timeout(None)         // Keep connections alive
    .http2_prior_knowledge()         // Skip HTTP/1.1 negotiation if server supports it
    .tcp_nodelay(true)               // Reduce local latency
    .build()?;

    Ok(())
}
