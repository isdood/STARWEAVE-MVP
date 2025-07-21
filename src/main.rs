// #FFD700 System Manifest (Enhanced)
mod concepts;
mod embedding;
mod actions;
mod state;

use concepts::{SimilarityEngine, cosine_similarity, ConceptVector};
use embedding::EmbeddingGenerator;
use actions::ActionSystem;
use state::StateUpdater;
use std::io;

fn main() {
    println!("🌟 STARWEAVE Vector Agent Initializing (Persistent Intelligence PoC)");

    // Initialize core components
    let mut engine = SimilarityEngine::new();
    let embedder = EmbeddingGenerator::new().unwrap();
    let mut action_system = ActionSystem::new();
    let mut state_updater = StateUpdater::new();

    println!("✅ {} concept vectors loaded", engine.concepts.len());
    println!("🚀 System ready for similarity analysis\n");
    println!("🔮 Internal reflection interval: every {} interactions\n", state_updater.reflection_interval);

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

        // Detect best matching concept (returns owned value)
        if let Some(concept) = engine.find_best_match(&embedding) {
            println!("\n✨ Best match: {}!", concept.name);
            println!("   Similarity: {:.2}", cosine_similarity(&concept.vector, &embedding));
            println!("   Curiosity score: {:.2}", concept.curiosity_score);
            println!("   State before update: [{:.3}, {:.3}]",
                concept.stochastic_state[0], concept.stochastic_state[1]);

            // Create mutable copy for state evolution
            let mut evolved_concept = concept.clone();

            // Evolve state
            state_updater.update_state(&mut evolved_concept);
            println!("   State after update:  [{:.3}, {:.3}]",
                evolved_concept.stochastic_state[0], evolved_concept.stochastic_state[1]);
            println!("   Updated curiosity:   {:.3}", evolved_concept.curiosity_score);

            // Trigger action
            let response = action_system.trigger_action(&evolved_concept, input);
            println!("\n💫 System action:\n{}\n", response);

            // Update original concept in engine
            engine.update_concept_after_interaction(&concept.name);
        } else {
            println!("\n🔍 No strong match found. Responding with default action.");
            println!("💬 I have processed your input about '{}'", input);
            action_system.trigger_action(
                &ConceptVector::default(),
                input
            );
        }

        // Trigger self-reflection periodically
        if state_updater.should_trigger_reflection() {
            println!("\n🌌 Internal Reflection Triggered:");
            println!("   Recent actions:");
            for action in action_system.get_recent_actions().iter().take(3) {
                println!("     - {}", action);
            }
            println!("   System state evolving...");
        }

        println!("⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯");
    }

    println!("\n🌌 STARWEAVE session completed.");
}
