
use rand::Rng;
use crate::config::{Config, Scenario};

#[derive(Debug, Clone)]
struct CachedScenario {
    
}

struct ScenarioPool {

}

#[derive(Debug, Clone)]
pub struct ScenarioCache {
    ScenarioPool: Vec<CachedScenario>,
    total_weight: u32,
}

impl ScenarioCache {
    pub fn from_confg(config: Config) -> Result<ScenarioCache, anyhow::Error> {
        // Calculate total weight of all scenarios
        total_weight = config.scenarios.values().map(|s| s.weight).sum();
    }

    pub fn get_next_scenario(&self) -> &Scenario {
        let mut rng = rand::rng();

        // Select and return a scenario according to the distribution: weight / total_weight
        let random_weight = rng.random_range(0..self.total_weight);
        let mut current_weight = 0;
        for (_name, scenario) in &self.scenarios {
            current_weight += scenario.weight;

            if current_weight > random_weight  { 
                return scenario
            }
        }

        self.scenarios.values().next().expect("Validation ensures one scenario is defined")
    }
}
