use std::collections::{
    HashMap,
    hash_map::Entry::{Occupied, Vacant},
};

use crate::results::model::{ProfileResults, ResultMatrix};

pub struct RequestStatistics {
    pub request_count: u64,
    pub total_response_time: u64,
    pub statuses: HashMap<u16, u64>,
}

pub struct ProfileStatistics {
    requests: HashMap<String, RequestStatistics>,
    // TODO Add clients hashmap for grouping request response stats by client
}

fn calculate_request_statistics(results: &ResultMatrix) -> HashMap<String, RequestStatistics> {
    let results_flat = results.into_iter().flatten();
    let mut request_map: HashMap<String, RequestStatistics> = HashMap::new();

    for result in results_flat {
        let request_url = result.request_url.clone();

        match request_map.entry(request_url) {
            Occupied(mut request_entry) => {
                let statistic = request_entry.get_mut();
                statistic.update(result.response_time, result.status);
            }
            Vacant(entry) => {
                let statistic = entry.insert(RequestStatistics::new());
                statistic.update(result.response_time, result.status);
            }
        }
    }

    request_map
}

impl RequestStatistics {
    pub fn new() -> RequestStatistics {
        Self {
            request_count: 0,
            total_response_time: 0,
            statuses: HashMap::new(),
        }
    }

    pub fn update(&mut self, response_time: u64, status: u16) {
        self.request_count += 1;
        self.total_response_time += response_time;

        match self.statuses.entry(status) {
            Occupied(count) => {
                let new_count = count.get() + 1;
                self.statuses.insert(status, new_count);
            }
            Vacant(_) => {
                self.statuses.insert(status, 1);
            }
        }
    }

    pub fn request_count(&self) -> &u64 {
        &self.request_count
    }

    pub fn avg_response_time(&self) -> u64 {
        self.total_response_time / self.request_count
    }

    pub fn status_rates(&self) -> HashMap<u16, f64> {
        let mut rates: HashMap<u16, f64> = HashMap::new();
        for (status, count) in self.statuses.iter() {
            rates.insert(
                status.clone(),
                count.clone() as f64 / self.request_count as f64,
            );
        }

        rates
    }
}

impl ProfileStatistics {
    pub fn get_request_statistics(&self) -> &HashMap<String, RequestStatistics> {
        &self.requests
    }
}

impl From<ProfileResults> for ProfileStatistics {
    fn from(profile_results: ProfileResults) -> Self {
        let results_matrix = profile_results.get_all_results();
        let requests = calculate_request_statistics(results_matrix);
        Self { requests }
    }
}
