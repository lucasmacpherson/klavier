use reqwest::{header::{HeaderMap, HeaderName, HeaderValue}, Client as HttpClient};
use std::sync::Arc;

use crate::config::{Config, Scenario};
use crate::scenarios::{ScenarioCache};

#[derive(Debug, Clone)]
pub struct Client {
    config: Arc<Config>,
    http_client: HttpClient,
    scenario_cache: ScenarioCache,
}

#[derive(Debug, Clone)]
pub struct RequestResult {
    pub status: u32,
    pub body: String,
    pub response_time: u64,
}

impl Client {
    pub fn build_client(config: Arc<Config>, http_client: HttpClient) -> Client {
        Client {
            config: config,
            http_client: http_client,
            scenario_cache: ScenarioCache,
        }
    } 
    
    fn send_request(&self, scenario: Scenario) -> Result<(), anyhow::Error> {
        let mut headers: HeaderMap = HeaderMap::new();
        for (key, val) in scenario.headers {
            let header_name = HeaderName::from_bytes(key.as_bytes())?;
            let header_value = HeaderValue::from_bytes(val.as_bytes())?;
            headers.insert(header_name, header_value);
        }


        Ok(())
    }

    pub fn run_next_scenario(&self) {
        let scenario: &Scenario = self.scenario_cache.get_next_scenario();
         
    }
}
