use anyhow::{Context, Result};
use polars::frame::DataFrame;
use std::env;

use klavier::{config::Config, loadtest::LoadTest, results::ProfileResults};

fn print_results(profile_results: &ProfileResults) -> Result<()> {
    for client_id in 0..profile_results.num_clients() {
        println!("================================================================");
        println!("Client {} Results", client_id);
        for result in profile_results.get_client_results(client_id)? {
            println!(
                "- Request: {} | Status: {} | Response Time: {}ms \n  Response body: {}",
                &result.request_url, &result.status, &result.response_time, &result.body
            );
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let config_path = args
        .get(1)
        .context(format!("Usage: {} <config-path> [num-clients]", args[0]))?;

    let client_n = args
        .get(2)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(1);

    let config = Config::from_filepath(config_path)?;
    let base_url = &config.target.base_url;
    println!(
        "Loaded config \"{}\" with target URL \"{}\"",
        &args[1], base_url
    );

    println!(
        "Running test with {} clients for {} seconds...",
        client_n, config.timings.test_duration_seconds
    );
    let test = LoadTest::new(config);
    let results = test.run(client_n).await?;

    //(print_results(&results))?;
    
    let df: DataFrame = results.into();
    println!("{}", df);

    Ok(())
}
