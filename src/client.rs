use reqwest::Client as HttpClient;
use std::sync::Arc;
use std::time::Instant;

use crate::config::{Config};
use crate::scenarios::{CachedScenario, ScenarioCache};

pub struct Client {
    config: Arc<Config>,
    http_client: HttpClient,
    scenario_cache: ScenarioCache,
}

#[derive(Debug, Clone)]
pub struct RequestResult {
    pub status: u16,
    pub body: String,
    pub response_time: u128,
}

impl Client {
    pub fn build_client(config: Arc<Config>, http_client: HttpClient) -> Result<Client, anyhow::Error> {
        Ok (Self {
            config: config.clone(),
            http_client: http_client,
            scenario_cache: ScenarioCache::from_config(config)?,
        })
    }
    
    pub async fn run_next_scenario(&mut self) -> Result<RequestResult, anyhow::Error> {
        let scenario: &CachedScenario = self.scenario_cache.get_next_scenario();
       // build request for the next scenario
        let mut request = self.http_client
            .request(scenario.method.clone(), &scenario.url)
            .headers(scenario.headers.clone());
            
        if let Some(body_bytes) = &scenario.body {
            request = request.body(body_bytes.clone())
        }

        println!("Sending request to {}:{}", &scenario.method, &scenario.url);
        // Start timer for response and send request
        let start = Instant::now();
        let response = request.send().await?;
        let duration_ms = start.elapsed().as_millis();
        
        Ok(RequestResult { 
            status: response.status().as_u16(),
            body: response.text().await?,
            response_time: duration_ms,
        })
    }
}
