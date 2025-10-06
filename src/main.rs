use std::env;
use anyhow::{bail, Result};

use klavier::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("Please provide a profile path (e.g. profiles/example.toml)")
    }

    let config = Config::from_filepath(&args[1])?;
    let base_url = &config.target.base_url;
    println!("Loaded config {} with target base_url {}", &args[1], base_url);

    let

    for result in results {
        println!("Status: {} | Response: {} | Response Time: {}ms", &result.status, &result.body, &result.response_time);
    }

    Ok(())
}
