use crate::results::model::{ProfileResults, RequestResult};

pub struct ProfileStatistics {
    results: Vec<Vec<RequestResult>>,
}

impl ProfileStatistics {
    
}

impl From<ProfileResults> for ProfileStatistics {
   fn from(profile_results: ProfileResults) -> Self {
       Self {
           results: profile_results.get_all_results(),
       }
   } 
}
