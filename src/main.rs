// #FFD700 System Manifest
mod concepts;
mod embedding;
mod actions;
mod state;

use concepts::{SimilarityEngine, cosine_similarity};
use embedding::EmbeddingGenerator;
use actions::ActionSystem;
use state::StateUpdater;
use std::io;

fn main() {
    println!("🌟 STARWEAVE Vector Agent Initializing");

    // Initialize core components
    let engine = SimilarityEngine::new();
    let embedder = EmbeddingGenerator::new().unwrap();
    let mut action_system = ActionSystem::new();
    let state_updater = StateUpdater::new();

    println!("✅ {} concept vectors loaded", engine.concepts.len());
    println!("🚀 System ready for similarity analysis\n");

    loop {
        println!("Enter a concept to analyze (or '/exit' to quit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "/exit" {
            break;
        }
        if input.is_empty() {
            continue;
        }

        // Generate embedding
        let embedding = embedder.embed(input).unwrap();

        // Detect best matching concept
        if let Some(concept) = engine.find_best_match(&embedding) {
            println!("\n✨ Best match: {}!", concept.name);
            println!("   Similarity: {:.2}", cosine_similarity(&concept.vector, &embedding));

            println!("   State before update: [{:.3}, {:.3}]",
                concept.stochastic_state[0], concept.stochastic_state[1]);

            // Create mutable copy for state evolution
            let mut evolved_concept = concept.clone();

            // Evolve state
            state_updater.update_state(&mut evolved_concept);
            println!("   State after update:  [{:.3}, {:.3}]",
                evolved_concept.stochastic_state[0], evolved_concept.stochastic_state[1]);

            // Trigger action
            let response = action_system.trigger_action(&evolved_concept, input);
            println!("\n💫 System action:\n{response}\n");

            // Integrate knowledge
            action_system.integrate_knowledge(input);
            println!("🧠 '{input}' added to working memory");
        } else {
            println!("\n🔍 No strong match found. Responding with default action.");
            println!("💬 I have processed your input about '{input}'");
        }

        println!("⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯");
    }

    println!("\n🌌 STARWEAVE session completed.");
}
