use anyhow::Error;

#[derive(Debug, Clone)]
pub struct RequestResult {
    pub timestamp: u128,
    pub request_url: String,
    pub status: u16,
    pub body: String,
    pub response_time: u128,
}

pub struct ProfileResults {
    results: Vec<Vec<RequestResult>>,
}

impl ProfileResults {
     pub fn new() -> Result<Self, Error> {
        Ok( Self { results: Vec::new() })
    }
}
