use std::collections::HashMap;
use std::fs;
use http::Method;
use toml;
use serde::{Deserialize, Serialize};
use anyhow::{bail, Context, Result};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Target {
    pub base_url: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Request {
    pub method: HttpMethod,
    pub path: String,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Scenario {
    pub weight: u32,
    pub requests: Vec<Request>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Timings {
    pub test_duration_seconds: u64,
    pub min_scenario_interval_ms: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    pub target: Target,
    pub scenarios: HashMap<String, Scenario>,
    pub timings: Timings,
}

impl From<HttpMethod> for Method {
    fn from(method: HttpMethod) -> Self {
        match method {
            HttpMethod::GET => Method::GET,
            HttpMethod::POST => Method::POST,
            HttpMethod::PUT => Method::PUT,
            HttpMethod::DELETE => Method::DELETE,
            HttpMethod::PATCH => Method::PATCH,
        }
    }
}

impl Config {
    pub fn validate(&self) -> Result<()> {
        // Validate the base URL is defined correctly
        if self.target.base_url.is_empty() || !self.target.base_url.starts_with("http") {
            bail!("A target base_url must be defined and begin with http:// or https://")
        }
        
        // Ensure at least one scenario has been defined
        if self.scenarios.is_empty() {
            bail!("At least one scenario must be defined")
        }
        Ok(())
    }

    pub fn from_filepath(filepath: &str) -> Result<Config> {
        let content: String = fs::read_to_string(filepath)
            .with_context(|| format!("Failed to read file at {}", filepath))?;
        let config: Config = toml::from_str(&content)
            .with_context(|| format!("File at {} is not a valid config! See example.toml", filepath))?;
        config.validate()?;
        Ok(config)
    }
}
