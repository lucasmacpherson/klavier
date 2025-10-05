use std::sync::Arc;

use anyhow::Error;
use bytes::Bytes;

use http::{HeaderMap, HeaderName, HeaderValue, Method};
use rand::{rngs::StdRng, Rng, SeedableRng};
use crate::config::{Config, Request, Scenario};

#[derive(Debug, Clone)]
pub struct RuntimeRequest {
    pub method: Method,
    pub url: String,
    pub headers: HeaderMap,
    pub body: Option<Bytes>,
}

#[derive(Debug, Clone)]
pub struct RuntimeScenario {
    pub requests: Vec<RuntimeRequest>,
}

#[derive(Debug, Clone)]
pub struct ScenarioPool {
    rng: StdRng,
    scenario_pool: Vec<RuntimeScenario>,
}

impl RuntimeRequest {
    pub fn new(request: Request, base_url: &str) -> Result<Self, Error> {
        let method: Method = request.method.into();
        let url: String = format!("{}{}", base_url, request.path);

        // Collect headers
        let mut headers: HeaderMap = HeaderMap::new();
        for (name, value) in request.headers {
            let header_name: HeaderName = name.parse()?;
            let header_value: HeaderValue = value.parse()?;
            headers.insert(header_name, header_value);
        }

        // Parse body optional if value exists
        let body = request.body.map(|json_str| Bytes::from(json_str.into_bytes()));
        Ok(Self { method, url, headers, body })
    }
}

impl RuntimeScenario {
    pub fn new(scenario: Scenario, base_url: &str) -> Result<Self, Error> {
        let requests = scenario.requests
            .into_iter()
            .map(|request| RuntimeRequest::new(request, base_url))
            .collect::<Result<Vec<_>, _>>()?;
            // Using '_' to infer type <Result<Vec<RuntimeRequest>, anyhow::Error>>
        Ok(Self { requests })
    }
}

impl ScenarioPool {
    pub fn from_config(config: Arc<Config>) -> Result<Self, Error> {
        let mut scenario_pool: Vec<RuntimeScenario> = Vec::new();
        
        // Create flat weighted distribution of scenarios
        for scenario in config.scenarios.values() {
            let runtime_scenario = RuntimeScenario::new(scenario.clone(), &config.target.base_url)?;
            for _ in 0..scenario.weight {
                scenario_pool.push(runtime_scenario.clone());
            }
        }
        
        Ok(Self { rng: StdRng::from_os_rng(), scenario_pool})
    }

    pub fn get_next_scenario(&mut self) -> &RuntimeScenario {
        // Select randomly from weighted scenario pool
        let choice = self.rng.random_range(0..self.scenario_pool.len());
        &self.scenario_pool[choice]
    }
}
