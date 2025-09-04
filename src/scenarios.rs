use std::sync::Arc;

use bytes::Bytes;

use http::{HeaderMap, HeaderName, HeaderValue, Method};
use rand::{rngs::ThreadRng, Rng};
use crate::config::{Config, Scenario};

#[derive(Debug, Clone)]
pub struct CachedScenario {
    pub method: Method,
    pub url: String,
    pub headers: HeaderMap,
    pub body: Option<Bytes>,
}

#[derive(Debug, Clone)]
pub struct ScenarioCache {
    rng: ThreadRng,
    scenario_pool: Vec<CachedScenario>,
}

impl CachedScenario {
    pub fn from_scenario(base_url: String, scenario: Scenario) -> Result<CachedScenario, anyhow::Error> {
        let method: Method = scenario.method.into();
        let url: String = format!("{}{}", base_url, scenario.path);
        
        // Collect headers
        let mut headers: HeaderMap = HeaderMap::new();
        for (name, value) in scenario.headers {
            let header_name: HeaderName = name.parse()?;
            let header_value: HeaderValue = value.parse()?;
            headers.insert(header_name, header_value);
        }

        // Parse body optional if value exists
        let body = scenario.body.map(|json_str| Bytes::from(json_str.into_bytes()));

        Ok(Self { 
            method: method,
            url: url,
            headers: headers,
            body: body,
        })
    }
}

impl ScenarioCache {
    pub fn from_config(config: Arc<Config>) -> Result<ScenarioCache, anyhow::Error> {
        let mut scenario_pool: Vec<CachedScenario> = Vec::new();
        
        // Create flat weighted distribution of scenarios
        for scenario in config.scenarios.values() {
            let cached_scenario = CachedScenario::from_scenario(config.target.base_url.clone(), scenario.clone())?;
            for _ in 0..scenario.weight {
                scenario_pool.push(cached_scenario.clone());
            }
        }

        Ok(Self {
            rng: rand::rng(),
            scenario_pool: scenario_pool,
        })

    }

    pub fn get_next_scenario(&mut self) -> &CachedScenario {
        // Select randomly from weighted scenario pool
        let choice = self.rng.random_range(0..self.scenario_pool.len());
        &self.scenario_pool[choice]
    }
}
