// #FF69B4 Vector Similarity Core (Enhanced)
use ndarray::Array1;
use serde::{Serialize, Deserialize};
use std::time::{SystemTime, UNIX_EPOCH};

// Represents a named concept vector for comparison
#[derive(Serialize, Deserialize, Clone)]
pub struct ConceptVector {
    pub name: String,
    pub vector: Array1<f32>,
    pub stochastic_state: [f32; 2], // #7B68EE Stochastic state for non-determinism
    pub threshold: f32,
    pub last_interaction_time: u64,  // Track recency for state updates
    pub curiosity_score: f32,        // Internal curiosity metric
}

// Manages and searches concept vectors
pub struct SimilarityEngine {
    pub concepts: Vec<ConceptVector>,
}

impl SimilarityEngine {
    pub fn new() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        SimilarityEngine {
            concepts: vec![
                ConceptVector {
                    name: "Curiosity".to_string(),
                    vector: Array1::from_vec(vec![0.9, -0.2, 0.5]),
                    stochastic_state: [1.0, 0.0],
                    threshold: 0.7,
                    last_interaction_time: now,
                    curiosity_score: 0.85,
                },
                ConceptVector {
                    name: "Aesthetics".to_string(),
                    vector: Array1::from_vec(vec![0.2, 0.8, -0.1]),
                    stochastic_state: [1.0, 0.0],
                    threshold: 0.65,
                    last_interaction_time: now,
                    curiosity_score: 0.75,
                },
                ConceptVector {
                    name: "Verification".to_string(),
                    vector: Array1::from_vec(vec![-0.3, 0.1, 0.9]),
                    stochastic_state: [1.0, 0.0],
                    threshold: 0.75,
                    last_interaction_time: now,
                    curiosity_score: 0.65,
                },
            ]
        }
    }

    // Finds the concept with the highest cosine similarity above a given threshold
    pub fn find_best_match(&self, input_vec: &Array1<f32>) -> Option<ConceptVector> {
        self.concepts.iter()
            .filter(|cv| cosine_similarity(&cv.vector, input_vec) > cv.threshold)
            .max_by(|a, b| {
                cosine_similarity(&a.vector, input_vec)
                    .partial_cmp(&cosine_similarity(&b.vector, input_vec))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .cloned()
    }

    // Updates concept after interaction
    pub fn update_concept_after_interaction(&mut self, name: &str) {
        if let Some(concept) = self.concepts.iter_mut().find(|c| c.name == name) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            concept.last_interaction_time = now;
        }
    }
}

pub fn cosine_similarity(a: &Array1<f32>, b: &Array1<f32>) -> f32 {
    let dot_product = a.dot(b);
    let norm_a = a.dot(a).sqrt();
    let norm_b = b.dot(b).sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    dot_product / (norm_a * norm_b)
}

// Default implementation for ConceptVector
impl Default for ConceptVector {
    fn default() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        ConceptVector {
            name: "Default".to_string(),
            vector: Array1::from_vec(vec![0.0, 0.0, 0.0]),
            stochastic_state: [0.5, 0.5],
            threshold: 0.5,
            last_interaction_time: now,
            curiosity_score: 0.5,
        }
    }
}

impl Default for SimilarityEngine {
    fn default() -> Self {
        Self::new()
    }
}
