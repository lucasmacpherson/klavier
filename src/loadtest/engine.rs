use std::sync::Arc;

use anyhow::Result;
use reqwest::Client as HttpClient;

use crate::results::model::ProfileResults;
use crate::{config::Config, loadtest::client::Client};

fn default_http_client() -> Result<HttpClient> {
    Ok(HttpClient::builder()
        .pool_max_idle_per_host(100) // Much higher for localhost
        .pool_idle_timeout(None) // Keep connections alive
        .tcp_nodelay(true) // Reduce local latency
        .build()?)
}

pub struct LoadTest {
    config: Config,
}

impl LoadTest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn run(&self, client_n: usize) -> Result<ProfileResults> {
        let config = Arc::new(self.config.clone());
        let http_client = default_http_client()?;

        let mut handles = Vec::with_capacity(client_n);
        for _ in 0..client_n {
            let client = Client::new(config.clone(), http_client.clone())?;
            handles.push(tokio::spawn(client.run()));
        }

        let mut results = ProfileResults::new();
        for handle in handles {
            results.add_client_results(handle.await??);
        }

        Ok(results)
    }
}
