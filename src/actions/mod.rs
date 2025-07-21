// #00CED1 Autonomous Action System
use crate::concepts::ConceptVector;
use std::collections::VecDeque;

pub struct ActionSystem {
    memory: VecDeque<String>,
}

impl ActionSystem {
    pub fn new() -> Self {
        Self {
            memory: VecDeque::with_capacity(100),
        }
    }

    pub fn trigger_action(&mut self, concept: &ConceptVector, input: &str) -> String {
        self.memory.push_back(input.to_string());

        match concept.name.as_str() {
            "Curiosity" => self.curiosity_action(input),
            "Aesthetics" => self.aesthetics_action(input),
            "Verification" => self.verification_action(input),
            _ => "Standard response generated.".to_string(),
        }
    }

    fn curiosity_action(&self, input: &str) -> String {
        format!("🔍 Curiosity matched. Researching deeper aspects of: {input}")
    }

    fn aesthetics_action(&self, input: &str) -> String {
        format!("🎨 Aesthetics matched. Considering artistic interpretations for: {input}")
    }

    fn verification_action(&self, input: &str) -> String {
        format!("🔬 Verification matched. Cross-referencing facts about: {input}")
    }

    pub fn integrate_knowledge(&mut self, new_info: &str) {
        if self.memory.len() == self.memory.capacity() {
            self.memory.pop_front();
        }
        self.memory.push_back(new_info.to_string());
    }
}
