// #7B68EE Stochastic State Management (Enhanced)
use crate::concepts::ConceptVector;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct StateUpdater {
    pub reflection_interval: u32,
    pub interaction_count: u32,
}

impl StateUpdater {
    pub fn new() -> Self {
        StateUpdater {
            reflection_interval: 5,  // Trigger reflection every 5 interactions
            interaction_count: 0,
        }
    }

    // Applies state evolution with curiosity decay/boost
    pub fn update_state(&self, vector: &mut ConceptVector) {
        let mut rng = rand::thread_rng();

        // Calculate time decay factor
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let time_diff = now - vector.last_interaction_time;
        let decay_factor = (time_diff as f32 / 3600.0).exp().recip(); // 1/e^(hours)

        // Apply decay to curiosity score
        vector.curiosity_score *= decay_factor;

        // Add stochastic drift with curiosity influence
        let curiosity_boost = 0.1 * vector.curiosity_score;
        vector.stochastic_state[0] += 0.01 * (rng.gen::<f32>() - 0.5) + curiosity_boost;
        vector.stochastic_state[1] -= 0.01 * (rng.gen::<f32>() - 0.5) - curiosity_boost;

        // Clamp values to valid range
        vector.stochastic_state[0] = vector.stochastic_state[0].clamp(0.0, 1.0);
        vector.stochastic_state[1] = vector.stochastic_state[1].clamp(0.0, 1.0);
        vector.curiosity_score = vector.curiosity_score.clamp(0.1, 1.0);
    }

    // Trigger self-reflection based on interaction count
    pub fn should_trigger_reflection(&mut self) -> bool {
        self.interaction_count += 1;
        if self.interaction_count % self.reflection_interval == 0 {
            self.interaction_count = 0;
            true
        } else {
            false
        }
    }
}

impl Default for StateUpdater {
    fn default() -> Self {
        Self::new()
    }
}
