use anyhow::Result;
use reqwest::Client as HttpClient;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use std::u64;
use tokio::time::sleep;

use crate::config::{Config, Timings};
use crate::loadtest::scenarios::{RuntimeRequest, ScenarioPool};
use crate::results::model::RequestResult;

pub struct Client {
    http_client: HttpClient,
    config_timings: Timings,
    scenario_pool: ScenarioPool,
}

impl Client {
    pub fn new(config: Arc<Config>, http_client: HttpClient) -> Result<Client> {
        Ok(Self {
            http_client: http_client,
            config_timings: config.timings.clone(),
            scenario_pool: ScenarioPool::from_config(config)?,
        })
    }

    pub async fn run(mut self) -> Result<Vec<RequestResult>> {
        let mut results = Vec::new();

        // Start timer for testing profile
        let test_start = Instant::now();
        let test_duration = Duration::from_secs(self.config_timings.test_duration_seconds);
        let min_interval =
            Duration::from_millis(self.config_timings.min_scenario_interval_ms.into());

        // Track duration of most recent scenarios
        let mut last_scenario_end = Instant::now();

        loop {
            if test_start.elapsed() >= test_duration {
                break;
            }

            // Ensure minimum interval between scenarios
            let last_scenario_interval = last_scenario_end.elapsed();
            if last_scenario_interval < min_interval {
                let wait_time = min_interval - last_scenario_interval;
                sleep(wait_time).await;
            }

            // Run next scenario and append results to Vec
            results.extend(self.run_next_scenario().await?);
            last_scenario_end = Instant::now();
        }

        Ok(results)
    }

    async fn make_request(&mut self, request: RuntimeRequest) -> Result<RequestResult> {
        // build request
        let mut http_request = self
            .http_client
            .request(request.method.clone(), &request.url)
            .headers(request.headers.clone());

        if let Some(body_bytes) = request.body {
            http_request = http_request.body(body_bytes)
        }

        println!("Sending {} request to {}", &request.method, &request.url);
        // Start timer for response and send request
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_millis()
            .min(u64::MAX as u128) as u64;
        let start = Instant::now();

        let (status, body) = match http_request.send().await {
            Ok(response) => {
                let status = response.status().as_u16();
                let body = response
                    .text()
                    .await
                    .unwrap_or_else(|e| format!("Failed to parse response body: {}", e));
                (status, body)
            }
            Err(e) => (0, format!("Connection error: {}", e)),
        };

        let duration_ms = start.elapsed().as_millis().min(u64::MAX as u128) as u64;

        Ok(RequestResult {
            timestamp: timestamp,
            request_url: request.url,
            status: status,
            body: body,
            response_time: duration_ms,
        })
    }

    async fn run_next_scenario(&mut self) -> Result<Vec<RequestResult>> {
        let requests: Vec<RuntimeRequest> = self.scenario_pool.get_next_scenario().requests.clone();

        let mut results: Vec<RequestResult> = Vec::new();
        for request in requests {
            let result = self.make_request(request).await?;
            results.push(result);
        }

        Ok(results)
    }
}
