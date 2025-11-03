use anyhow::{Context, Result};
use klavier::{config::Config, results::output::get_header_string};
use klavier::loadtest::engine::LoadTest;
use klavier::results::model::ProfileResults;
use klavier::results::output::print_request_statistics;
use klavier::results::statistics::ProfileStatistics;
use std::{env, usize};

fn parse_args(args: Vec<String>) -> Result<Arguments> {

    Ok(Arguments { config_path, client_n })
}

fn load_config(filepath: &str) -> Result<Config> {
    let config = Config::new(filepath)?;
    println!(
        "Loaded config \"{}\" with target URL \"{}\"",
        filepath, &config.target.base_url
    );

    Ok(config)
}

async fn run_loadtest(config: Config, client_n: usize) -> Result<ProfileResults> {
    println!(
        "Running test with {} clients for {} seconds...",
        client_n, config.timings.test_duration_seconds
    );

    let test = LoadTest::new(config);
    let results = test.run(client_n).await?;

    println!(
        "{}\nTest complete \n", get_header_string()
    );
    Ok(results)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = parse_args(env::args().collect())?;
    let config = load_config(&args.config_path)?;

    let results = run_loadtest(config, args.client_n).await?;

    let statistics: ProfileStatistics = results.into();
    print_request_statistics(statistics);

    Ok(())
}
