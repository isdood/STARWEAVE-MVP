//! Library crate for STARWEAVE-MVP

pub mod concepts;
pub mod embedding;
pub mod actions;
pub mod state;
pub mod module_agent;
pub mod agent_orchestrator;

// Re-export public API
pub use concepts::{ConceptVector, SimilarityEngine, cosine_similarity};
pub use embedding::EmbeddingGenerator;
pub use actions::ActionSystem;
pub use state::StateUpdater;
pub use module_agent::ModuleAgent;
pub use agent_orchestrator::AgentOrchestrator;
