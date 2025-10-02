use std::sync::Arc;

use anyhow::Error;
use reqwest::Client as HttpClient;
use tokio::task::JoinHandle;

use crate::{config::Config, results::{ProfileResults, RequestResult}};

pub struct LoadTest {
    config: Config,
}

impl LoadTest {
    fn default_http_client() -> Result<HttpClient, Error> {
        Ok(HttpClient::builder()
        .pool_max_idle_per_host(100)     // Much higher for localhost
        .pool_idle_timeout(None)         // Keep connections alive
        .tcp_nodelay(true)               // Reduce local latency
        .build()?)
    }

    fn run(&self, client_n: u32) -> Result<ProfileResults, Error> {
        let config = Arc::new(self.config);
        
        let mut handles: Vec<JoinHandle<Vec<RequestResult>>> = Vec::new();
        
        :wq
            :wq

    }
}
