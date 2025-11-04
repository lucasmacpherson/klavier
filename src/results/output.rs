use crate::results::model::ProfileResults;
use crate::results::statistics::ProfileStatistics;
use crate::results::wrapper::ProfileDataFrame;
use anyhow::Result;
use polars::{frame::DataFrame, io::SerWriter, prelude::CsvWriter};
use std::fs::File;

pub fn get_header_string() -> String {
    "================================================================".to_string()
}

pub fn print_results(profile_results: &ProfileResults) -> Result<()> {
    for client_id in 0..profile_results.num_clients() {
        println!("{}", get_header_string());
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

pub fn save_results_to_csv(profile_results: ProfileResults, filepath: String) -> Result<()> {
    let stats: ProfileDataFrame = profile_results.into();
    let mut df: DataFrame = stats.results;

    let mut file = File::create(filepath)?;
    CsvWriter::new(&mut file)
        .include_header(true)
        .with_separator(b',')
        .with_quote_char(b'"') // Properly quote strings with commas
        .with_line_terminator("\r\n".to_string()) // Windows line endings for Excel
        .finish(&mut df)?;

    Ok(())
}

pub fn print_request_statistics(profile_stats: ProfileStatistics) {
    for (request_url, stats) in profile_stats.get_request_statistics().iter() {
        println!("{}", get_header_string());
        println!(
            "\nEndpoint: {} ({} requests)",
            request_url,
            stats.request_count()
        );
        println!("Avg Response Time: {}ms", stats.avg_response_time());
        println!("Status Codes:");
        for (code, rate) in stats.status_rates() {
            println!("- HTTP {} -> {}%", code, rate * 100 as f64)
        }
    }
    println!(
        "Combined Average Response Time (All Requests): {}",
        profile_stats.get_combined_avg_response_time()
    );
}

pub fn save_request_statistics_to_csv(profile_stats: ProfileStatistics, filepath: String) {}
