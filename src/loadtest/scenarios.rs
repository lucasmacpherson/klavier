use std::sync::Arc;

use anyhow::Result;
use bytes::Bytes;

use crate::config::{Config, Request, Scenario};
use http::{HeaderMap, HeaderName, HeaderValue, Method};
use rand::{Rng, SeedableRng, rngs::StdRng};

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
    pub fn new(request: Request, base_url: &str) -> Result<Self> {
        let method = request.method.into();
        let url = format!("{}{}", base_url, request.path);
        let mut headers = request.headers.unwrap_or_default();

        // Add Klavier User-Agent header
        headers.insert("User-Agent".into(), "Klavier/1.0".into());

        // Collect headers in http::HeaderMap struct
        let mut header_map: HeaderMap = HeaderMap::new();
        for (name, value) in headers {
            let header_name: HeaderName = name.parse()?;
            let header_value: HeaderValue = value.parse()?;
            header_map.insert(header_name, header_value);
        }

        // Convert JSON body string optional to bytes
        let body = request.body.map(Bytes::from);
        Ok(Self {
            method,
            url,
            headers: header_map,
            body,
        })
    }
}

impl RuntimeScenario {
    pub fn new(scenario: Scenario, base_url: &str) -> Result<Self> {
        let requests = scenario
            .requests
            .into_iter()
            .map(|request| RuntimeRequest::new(request, base_url))
            .collect::<Result<Vec<_>, _>>()?;
        // Using '_' to infer type <Result<Vec<RuntimeRequest>, anyhow::Error>>
        Ok(Self { requests })
    }
}

impl ScenarioPool {
    // TODO !! Store unique scenarios in HashSet and built weighted pool of pointers
    // Don't clone! Why the hell did we do this...
    pub fn from_config(config: Arc<Config>) -> Result<Self> {
        let mut scenario_pool: Vec<RuntimeScenario> = Vec::new();

        // Create flat weighted distribution of scenarios
        for scenario in config.scenarios.values() {
            let runtime_scenario = RuntimeScenario::new(scenario.clone(), &config.target.base_url)?;
            for _ in 0..scenario.weight {
                scenario_pool.push(runtime_scenario.clone());
            }
        }

        Ok(Self {
            rng: StdRng::from_os_rng(),
            scenario_pool,
        })
    }

    pub fn get_next_scenario(&mut self) -> &RuntimeScenario {
        // Select randomly from weighted scenario pool
        let choice = self.rng.random_range(0..self.scenario_pool.len());
        &self.scenario_pool[choice]
    }
}
