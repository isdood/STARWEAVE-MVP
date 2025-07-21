// #FFD700 System Manifest (Enhanced with Module Agents)
use starweave_mvp::concepts::{SimilarityEngine, cosine_similarity, ConceptVector};
use starweave_mvp::embedding::EmbeddingGenerator;
use starweave_mvp::actions::ActionSystem;
use starweave_mvp::state::StateUpdater;
use starweave_mvp::module_agent::ModuleAgent;
use ndarray::Array1;
use std::io;

fn main() {
    println!("🌟 STARWEAVE Vector Agent Initializing (Modular AI PoC)");

    // Initialize core components
    let mut engine = SimilarityEngine::new();
    let embedder = EmbeddingGenerator::new().unwrap();
    let mut action_system = ActionSystem::new();
    let mut state_updater = StateUpdater::new();

    // Create specialized modules using concept names
    let curiosity_concepts = engine.concepts.iter()
        .filter(|c| c.name == "Curiosity")
        .cloned()
        .collect();
    let curiosity_module = ModuleAgent::new("Curiosity", curiosity_concepts);

    let aesthetics_concepts = engine.concepts.iter()
        .filter(|c| c.name == "Aesthetics")
        .cloned()
        .collect();
    let aesthetics_module = ModuleAgent::new("Aesthetics", aesthetics_concepts);

    let verification_concepts = engine.concepts.iter()
        .filter(|c| c.name == "Verification")
        .cloned()
        .collect();
    let verification_module = ModuleAgent::new("Verification", verification_concepts);

    // Register all modules with orchestrator
    action_system.orchestrator.register_module(curiosity_module);
    action_system.orchestrator.register_module(aesthetics_module);
    action_system.orchestrator.register_module(verification_module);

    println!("✅ {} concept vectors loaded", engine.concepts.len());
    println!("🚀 {} specialized modules registered", action_system.orchestrator.modules.len());
    println!("   - Curiosity\n   - Aesthetics\n   - Verification");
    println!("🔮 Co-creation propensity: {:.1}%", action_system.orchestrator.propensity_to_co_create * 100.0);
    println!("💡 Proactive prompts available: {}", action_system.orchestrator.proactive_prompts.len());
    println!("🤝 Co-creation mode: {}\n", if action_system.co_creation_mode { "ENABLED" } else { "DISABLED" });

    let mut interaction_count = 0;

    loop {
        println!("Enter a concept to analyze (or type command: /co-create, /exit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        // Handle commands before processing input
        if input == "/exit" {
            break;
        }

        // Handle co-creation toggle command
        if input == "/co-create" {
            action_system.toggle_co_creation();
            println!("\n🔄 Co-creation mode {}",
                     if action_system.co_creation_mode { "ENABLED" } else { "DISABLED" });
            println!("⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯");
            continue;
        }

        // Generate embedding
        let embedding = match embedder.embed(input) {
            Ok(emb) => emb,
            Err(e) => {
                println!("\n⚠️ Embedding error: {e}. Using default vector.");
                Array1::zeros(384) // Use a default vector if embedding fails
            }
        };

        // Detect best matching concept
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
            println!("\n💫 System action:\n{response}\n");

            // Update original concept in engine
            engine.update_concept_after_interaction(&concept.name);
        } else {
            println!("\n🔍 No strong match found. Responding with default action.");
            println!("💬 I have processed your input about '{input}'");
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
                println!("     - {action}");
            }
            println!("   System state evolving...");
        }

        // Generate proactive prompts occasionally
        interaction_count += 1;
        if interaction_count % 5 == 0 {
            let prompt = action_system.orchestrator.generate_proactive_prompt();
            println!("\n💡 Proactive Prompt: {prompt}");
        }

        println!("⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯⎯");
    }

    println!("\n🌌 STARWEAVE session completed.");

    // Print co-creation statistics
    println!("\n🤝 Co-Creation Statistics:");
    for (name, module) in &action_system.orchestrator.modules {
        println!("   Module '{name}': {} co-creations", module.co_creation_count);
    }
}
