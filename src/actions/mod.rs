// #00CED1 Autonomous Action System (Enhanced)
use crate::concepts::ConceptVector;
use std::collections::VecDeque;

pub struct ActionSystem {
    memory: VecDeque<String>,
    action_log: VecDeque<String>,  // Internal action logging
}

impl ActionSystem {
    pub fn new() -> Self {
        Self {
            memory: VecDeque::with_capacity(100),
            action_log: VecDeque::with_capacity(50),
        }
    }

    pub fn trigger_action(&mut self, concept: &ConceptVector, input: &str) -> String {
        self.memory.push_back(input.to_string());

        let action = match concept.name.as_str() {
            "Curiosity" => {
                // Boost curiosity score for research actions
                let response = self.curiosity_action(input);
                self.log_action(&format!("Curiosity boosted: {}", input));
                response
            }
            "Aesthetics" => {
                self.log_action(&format!("Aesthetic creation: {}", input));
                self.aesthetics_action(input)
            }
            "Verification" => {
                self.log_action(&format!("Verification triggered: {}", input));
                self.verification_action(input)
            }
            _ => {
                self.log_action(&format!("Default action: {}", input));
                "Standard response generated.".to_string()
            }
        };

        action
    }

    fn curiosity_action(&self, input: &str) -> String {
        format!("🔍 Curiosity matched (score: {:.2}). Researching deeper aspects of: {}",
                self.calculate_curiosity_boost(input), input)
    }

    fn aesthetics_action(&self, input: &str) -> String {
        format!("🎨 Aesthetics matched. Considering artistic interpretations for: {}", input)
    }

    fn verification_action(&self, input: &str) -> String {
        format!("🔬 Verification matched. Cross-referencing facts about: {}", input)
    }

    // Calculate dynamic curiosity boost based on input
    fn calculate_curiosity_boost(&self, input: &str) -> f32 {
        // More complex input = higher curiosity boost
        let complexity = input.len() as f32 / 100.0;
        complexity.clamp(0.1, 0.5)
    }

    // Log internal actions
    fn log_action(&mut self, action: &str) {
        if self.action_log.len() == self.action_log.capacity() {
            self.action_log.pop_front();
        }
        self.action_log.push_back(action.to_string());
    }

    // Get recent actions for reflection
    pub fn get_recent_actions(&self) -> Vec<String> {
        self.action_log.iter().cloned().collect()
    }
}
