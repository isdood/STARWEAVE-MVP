// #ADD8E6 Module Agent Definition
use crate::concepts::{ConceptVector, SimilarityEngine};
use ndarray::Array1;

pub struct ModuleAgent {
    pub name: String,
    pub concepts: Vec<ConceptVector>,
    pub local_engine: SimilarityEngine,
    pub co_creation_count: u32,
}

impl ModuleAgent {
    pub fn new(name: &str, concepts: Vec<ConceptVector>) -> Self {
        let local_engine = SimilarityEngine { concepts: concepts.clone() };
        ModuleAgent {
            name: name.to_string(),
            concepts,
            local_engine,
            co_creation_count: 0,
        }
    }

    // Process input within this module's context
    pub fn process_input(&mut self, input_vec: &Array1<f32>) -> Option<ConceptVector> {
        self.local_engine.find_best_match(input_vec)
    }

    // Suggest a concept to another module based on implicit connections
    pub fn suggest_concept(&self, _other: &str) -> Option<&ConceptVector> {
        // Simple implicit suggestion: find the concept with highest curiosity
        self.concepts.iter()
            .max_by(|a, b| a.curiosity_score.partial_cmp(&b.curiosity_score).unwrap())
    }

    // Record a successful co-creation
    pub fn record_co_creation(&mut self) {
        self.co_creation_count += 1;
    }
}
