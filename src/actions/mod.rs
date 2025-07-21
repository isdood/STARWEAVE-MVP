// #00CED1 Autonomous Action System (Enhanced with Co-Creation)
use crate::concepts::ConceptVector;
use crate::agent_orchestrator::AgentOrchestrator;
use std::collections::VecDeque;

pub struct ActionSystem {
    memory: VecDeque<String>,
    action_log: VecDeque<String>,
    pub orchestrator: AgentOrchestrator,
    pub co_creation_mode: bool,
}

impl ActionSystem {
    pub fn new() -> Self {
        Self {
            memory: VecDeque::with_capacity(100),
            action_log: VecDeque::with_capacity(50),
            orchestrator: AgentOrchestrator::new(),
            co_creation_mode: false,
        }
    }

    pub fn trigger_action(&mut self, concept: &ConceptVector, input: &str) -> String {
        self.memory.push_back(input.to_string());

        let action = match concept.name.as_str() {
            "Curiosity" => {
                let response = self.curiosity_action(input);
                self.log_action(&format!("[Curiosity] Researching: {input}"));
                response
            }
            "Aesthetics" => {
                self.log_action(&format!("[Aesthetics] Creating: {input}"));
                self.aesthetics_action(input)
            }
            "Verification" => {
                self.log_action(&format!("[Verification] Verifying: {input}"));
                self.verification_action(input)
            }
            _ => {
                self.log_action(&format!("[Default] Processing: {input}"));
                "Standard response generated.".to_string()
            }
        };

        // Add co-creation if enabled
        if self.co_creation_mode {
            let co_creation = self.orchestrator.co_create(&concept.name, input);
            format!("{action}\n\n🤝 Co-Creation:\n{co_creation}")
        } else {
            action
        }
    }

    fn curiosity_action(&self, input: &str) -> String {
        format!(
            "🔍 Curiosity matched (score: {:.2}). Researching deeper aspects of: {input}",
            self.calculate_curiosity_boost(input)
        )
    }

    fn aesthetics_action(&self, input: &str) -> String {
        format!("🎨 Aesthetics matched. Considering artistic interpretations for: {input}")
    }

    fn verification_action(&self, input: &str) -> String {
        format!("🔬 Verification matched. Cross-referencing facts about: {input}")
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

    // Toggle co-creation mode
    pub fn toggle_co_creation(&mut self) {
        self.co_creation_mode = !self.co_creation_mode;
        let status = if self.co_creation_mode { "ENABLED" } else { "DISABLED" };
        self.log_action(&format!("Co-creation mode {status}"));
    }
}

impl Default for ActionSystem {
    fn default() -> Self {
        Self::new()
    }
}
