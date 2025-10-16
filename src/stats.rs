use polars::{df, frame::DataFrame};

use crate::results::ProfileResults;

pub struct ProfileStatistics {
    pub results: DataFrame,
}

impl ProfileStatistics {

}

impl From<ProfileResults> for ProfileStatistics {
   fn from(profile_results: ProfileResults) -> Self {
        let total_rows = profile_results.get_all_results().into_iter().map(|v| v.len()).sum();

        // Pre-allocate vectors with size total_rows
        let mut client_ids = Vec::with_capacity(total_rows);
        let mut timestamps = Vec::with_capacity(total_rows);
        let mut urls = Vec::with_capacity(total_rows);
        let mut statuses = Vec::with_capacity(total_rows);
        let mut bodies = Vec::with_capacity(total_rows);
        let mut response_times = Vec::with_capacity(total_rows);

        // Can drop ProfileResults struct entirely and just pass Vec<Vec<_>> 
        // to ensure cleaner ownership and avoid unecessary cloning if no
        // new logic is added to the struct

        for (client_id, request_results) in profile_results.get_all_results().into_iter().enumerate() {
            for result in request_results {
                client_ids.push(client_id as u32);
                timestamps.push(result.timestamp);
                urls.push(result.request_url.clone());
                statuses.push(result.status as u32);
                bodies.push(result.body.clone());
                response_times.push(result.response_time);
            }
        }

        let df = df!(
            "client_id" => client_ids,
            "timestamp" => timestamps,
            "url" => urls,
            "status" => statuses,
            "body" => bodies,
            "response_time" => response_times
        ).unwrap();

        ProfileStatistics {
            results: df
        }
    }
}
