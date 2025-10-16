use anyhow::{Context, Result};

#[derive(Debug, Clone)]
pub struct RequestResult {
    pub timestamp: u64,
    pub request_url: String,
    pub status: u16,
    pub body: String,
    pub response_time: u64,
}

#[derive(Debug, Clone)]
pub struct ProfileResults {
    results: Vec<Vec<RequestResult>>,
}

impl ProfileResults {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }

    pub fn add_client_results(&mut self, client_results: Vec<RequestResult>) {
        self.results.push(client_results);
    }

    pub fn num_clients(&self) -> usize {
        self.results.len().clone()
    }

    pub fn get_client_results(&self, client_idx: usize) -> Result<&Vec<RequestResult>> {
        self.results
            .get(client_idx)
            .context("Client index out of bounds")
    }

    pub fn get_all_results(&self) -> &Vec<Vec<RequestResult>> {
        &self.results
    }
}
