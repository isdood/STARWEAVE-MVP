// #FFA07A Agent Orchestrator
use crate::module_agent::ModuleAgent;
use crate::concepts::cosine_similarity;
use ndarray::Array1;
use std::collections::HashMap;

pub struct AgentOrchestrator {
    pub modules: HashMap<String, ModuleAgent>,
    pub propensity_to_co_create: f32,
    pub proactive_prompts: Vec<String>,
}

impl AgentOrchestrator {
    pub fn new() -> Self {
        let proactive_prompts = vec![
            "What would happen if we combined these concepts?".to_string(),
            "How might we approach this from a different perspective?".to_string(),
            "What underlying patterns connect these ideas?".to_string(),
        ];

        AgentOrchestrator {
            modules: HashMap::new(),
            propensity_to_co_create: 0.3,
            proactive_prompts,
        }
    }

    // Register a module with the orchestrator
    pub fn register_module(&mut self, module: ModuleAgent) {
        self.modules.insert(module.name.clone(), module);
    }

    // Route input to the best module
    pub fn route_input(&mut self, input_vec: &Array1<f32>) -> Option<String> {
        let mut best_match: Option<(&String, f32)> = None;

        for (name, module) in &mut self.modules {
            if let Some(concept) = module.process_input(input_vec) {
                let similarity = cosine_similarity(&concept.vector, input_vec);

                if best_match.map(|(_, s)| similarity > s).unwrap_or(true) {
                    best_match = Some((name, similarity));
                }
            }
        }

        best_match.map(|(name, _)| name.clone())
    }

    // Attempt co-creation between modules
    pub fn co_create(&mut self, primary_module: &str, input: &str) -> String {
        let mut result = String::new();

        // Collect suggestions first to avoid borrow conflicts
        let mut suggestions = Vec::new();
        let primary_exists = self.modules.contains_key(primary_module);

        if !primary_exists {
            return "⚠️ Primary module not found\n".to_string();
        }

        result.push_str(&format!(
            "🧠 Primary module '{primary_module}' processing: {input}\n"
        ));

        // Find another module to co-create with
        for name in self.modules.keys().filter(|&n| n != primary_module).cloned().collect::<Vec<_>>() {
            if let Some(module) = self.modules.get_mut(&name) {
                if let Some(suggestion) = module.suggest_concept(primary_module) {
                    suggestions.push((name, suggestion.name.clone()));
                }
            }
        }

        // Process suggestions and record co-creations
        if !suggestions.is_empty() {
            for (name, suggestion) in &suggestions {
                result.push_str(&format!("💡 Module '{name}' suggests: {suggestion}\n"));

                if let Some(module) = self.modules.get_mut(name) {
                    module.record_co_creation();
                }

                if let Some(primary) = self.modules.get_mut(primary_module) {
                    primary.record_co_creation();
                }
            }

            // Increase propensity after successful co-creation
            self.propensity_to_co_create = (self.propensity_to_co_create + 0.1).min(0.9);
        } else {
            result.push_str("🔍 No co-creation suggestions available\n");
        }

        result
    }

    // Generate a proactive prompt
    pub fn generate_proactive_prompt(&self) -> &str {
        let index = (self.propensity_to_co_create * self.proactive_prompts.len() as f32) as usize;
        self.proactive_prompts.get(index).unwrap_or(&self.proactive_prompts[0])
    }
}

impl Default for AgentOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}
