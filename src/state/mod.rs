// #7B68EE Stochastic State Management
use crate::concepts::ConceptVector;
use rand::Rng;

pub struct StateUpdater;

impl StateUpdater {
    pub fn new() -> Self {
        StateUpdater
    }

    // Applies a small random change to a vector's state
    pub fn update_state(&self, vector: &mut ConceptVector) {
        let mut rng = rand::thread_rng();
        vector.stochastic_state[0] += 0.01 * (rng.gen::<f32>() - 0.5);
        vector.stochastic_state[1] -= 0.01 * (rng.gen::<f32>() - 0.5);
    }
}
