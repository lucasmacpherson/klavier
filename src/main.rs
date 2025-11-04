use anyhow::Result;
use clap::Parser;
use klavier::loadtest::engine::LoadTest;
use klavier::results::model::ProfileResults;
use klavier::results::output::{print_request_statistics, save_results_to_csv};
use klavier::results::statistics::ProfileStatistics;
use klavier::{config::Config, results::output::get_header_string};

#[derive(Parser, Debug)]
pub struct Arguments {
    /// Path to valid test profile TOML file
    pub config_path: String,
    /// Number of parallel clients
    #[arg(default_value = "1")]
    pub client_n: usize,
    /// Path to CSV file to output all response results
    #[arg(short, long, default_value = None)]
    pub results_out: Option<String>,
    /// Path to CSV file to output response statistics
    #[arg(short, long, default_value = None)]
    pub stats_out: Option<String>,
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

    println!("{} \n\nTest complete \n", get_header_string());
    Ok(results)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Arguments::parse();
    let config = load_config(&args.config_path)?;

    let results = run_loadtest(config, args.client_n).await?;

    if let Some(results_out) = args.results_out {
        save_results_to_csv(results.clone(), results_out)?;
    }

    let statistics: ProfileStatistics = results.into();
    print_request_statistics(statistics);

    Ok(())
}
