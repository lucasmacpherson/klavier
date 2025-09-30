use anyhow::Error;
use reqwest::Client as HttpClient;
use std::sync::Arc;
use std::time::Instant;

use crate::config::{Config};
use crate::scenarios::{RuntimeRequest, ScenarioPool};

pub struct Client {
    config: Arc<Config>,
    http_client: HttpClient,
    scenario_pool: ScenarioPool,
}

#[derive(Debug, Clone)]
pub struct RequestResult {
    pub status: u16,
    pub body: String,
    pub response_time: u128,
}

impl Client {
    pub fn build_client(config: Arc<Config>, http_client: HttpClient) -> Result<Client, Error> {
        Ok (Self {
            config: config.clone(),
            http_client: http_client,
            scenario_pool: ScenarioPool::from_config(config)?,
        })
    }
    
    pub async fn make_request(&mut self, request: RuntimeRequest) -> Result<RequestResult, Error> {
       // build request
         let mut http_request = self.http_client
            .request(request.method.clone(), &request.url)
            .headers(request.headers.clone());
            
        if let Some(body_bytes) = &request.body {
            http_request = http_request.body(body_bytes.clone())
        }

        println!("Sending request to {}:{}", &request.method, &request.url);
        // Start timer for response and send request
        let start = Instant::now();
        let response = http_request.send().await?;
        let duration_ms = start.elapsed().as_millis();
        
        Ok(RequestResult { 
            status: response.status().as_u16(),
            body: response.text().await?,
            response_time: duration_ms,
        })
    }

    pub async fn run_next_scenario(&mut self) -> Result<Vec<RequestResult>, Error> {
        let requests: Vec<RuntimeRequest> = self.scenario_pool.get_next_scenario().requests.clone();

        let mut results: Vec<RequestResult> = Vec::new();
        for request in requests {
            results.push(self.make_request(request).await?);
        }

        Ok(results)
    }
}
