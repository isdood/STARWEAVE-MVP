// #FF69B4 Vector Similarity Core
use ndarray::Array1;
use serde::{Serialize, Deserialize};

// Represents a named concept vector for comparison
#[derive(Serialize, Deserialize, Clone)]
pub struct ConceptVector {
    pub name: String,
    pub vector: Array1<f32>,
    pub stochastic_state: [f32; 2], // #7B68EE Stochastic state for non-determinism
    pub threshold: f32,
}

// Manages and searches concept vectors
pub struct SimilarityEngine {
    pub concepts: Vec<ConceptVector>,
}

impl SimilarityEngine {
    pub fn new() -> Self {
        SimilarityEngine {
            concepts: vec![
                ConceptVector {
                    name: "Curiosity".to_string(),
                    vector: Array1::from_vec(vec![0.9, -0.2, 0.5]),
                    stochastic_state: [1.0, 0.0],
                    threshold: 0.7,
                },
                ConceptVector {
                    name: "Aesthetics".to_string(),
                    vector: Array1::from_vec(vec![0.2, 0.8, -0.1]),
                    stochastic_state: [1.0, 0.0],
                    threshold: 0.65,
                },
                ConceptVector {
                    name: "Verification".to_string(),
                    vector: Array1::from_vec(vec![-0.3, 0.1, 0.9]),
                    stochastic_state: [1.0, 0.0],
                    threshold: 0.75,
                },
            ]
        }
    }

    // Finds the concept with the highest cosine similarity above a given threshold
    pub fn find_best_match<'a>(&'a self, input_vec: &Array1<f32>) -> Option<&'a ConceptVector> {
        self.concepts.iter()
            .filter(|cv| cosine_similarity(&cv.vector, input_vec) > cv.threshold)
            .max_by(|a, b| {
                cosine_similarity(&a.vector, input_vec)
                    .partial_cmp(&cosine_similarity(&b.vector, input_vec))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
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
