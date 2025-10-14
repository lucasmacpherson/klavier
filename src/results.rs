use anyhow::{Context, Result};
use polars::{df, frame::DataFrame};

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
}

impl From<ProfileResults> for DataFrame {
   fn from(profile_results: ProfileResults) -> Self {
        let total_rows = profile_results.results.iter().map(|v| v.len()).sum();

        // Pre-allocate vectors with size total_rows
        let mut client_ids = Vec::with_capacity(total_rows);
        let mut timestamps = Vec::with_capacity(total_rows);
        let mut urls = Vec::with_capacity(total_rows);
        let mut statuses = Vec::with_capacity(total_rows);
        let mut bodies = Vec::with_capacity(total_rows);
        let mut response_times = Vec::with_capacity(total_rows);

        for (client_id, request_results) in profile_results.results.into_iter().enumerate() {
            for result in request_results {
                client_ids.push(client_id as u32);
                timestamps.push(result.timestamp);
                urls.push(result.request_url);
                statuses.push(result.status as u32);
                bodies.push(result.body);
                response_times.push(result.response_time);
            }
        }

        df!(
            "client_id" => client_ids,
            "timestamp" => timestamps,
            "url" => urls,
            "status" => statuses,
            "body" => bodies,
            "response_time" => response_times
        ).unwrap()
    }
}
