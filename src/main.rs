use anyhow::{Context, Result};
use klavier::results::statistics::ProfileStatistics;
use polars::{frame::DataFrame, io::SerWriter, prelude::CsvWriter};
use std::{env, fs::File};

use klavier::config::Config;
use klavier::loadtest::engine::LoadTest;
use klavier::results::model::ProfileResults;
use klavier::results::wrapper::ProfileDataFrame;

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

fn save_results_to_csv(profile_results: ProfileResults) -> Result<()> {
    let stats: ProfileDataFrame = profile_results.into();
    let mut df: DataFrame = stats.results;

    let mut file = File::create("latest.csv")?;
    CsvWriter::new(&mut file)
        .include_header(true)
        .with_separator(b',')
        .with_quote_char(b'"') // Properly quote strings with commas
        .with_line_terminator("\r\n".to_string()) // Windows line endings for Excel
        .finish(&mut df)?;

    Ok(())
}

fn print_request_statistics(profile_stats: ProfileStatistics) {
    for (request_url, stats) in profile_stats.get_request_statistics().iter() {
        println!("================================================================");
        println!("Endpoint: {} ({} requests)", request_url, stats.request_count());
        println!("Avg Response Time: {}ms", stats.avg_response_time());
        println!("Status Codes:");
        for (code, rate) in stats.status_rates() {
            println!("- HTTP {} -> {}%", code, rate * 100 as f64)
        }
    }
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

    let statistics: ProfileStatistics = results.into();
    print_request_statistics(statistics);

    Ok(())
}
