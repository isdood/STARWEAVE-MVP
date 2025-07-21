// #7B68EE State Updater Tests
use starweave_mvp::state::StateUpdater;
use starweave_mvp::concepts::ConceptVector;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn test_state_evolution() {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let mut concept = ConceptVector {
        name: "Test".to_string(),
        vector: ndarray::Array1::from_vec(vec![0.5, 0.5, 0.5]),
        stochastic_state: [0.5, 0.5],
        threshold: 0.6,
        last_interaction_time: now - 3600, // 1 hour ago
        curiosity_score: 0.8,
    };

    let updater = StateUpdater::new();
    updater.update_state(&mut concept);

    // State should stay within [0,1] bounds
    assert!(concept.stochastic_state[0] >= 0.0 && concept.stochastic_state[0] <= 1.0);
    assert!(concept.stochastic_state[1] >= 0.0 && concept.stochastic_state[1] <= 1.0);
    assert!(concept.curiosity_score >= 0.1 && concept.curiosity_score <= 1.0);

    // Curiosity should decay over time
    assert!(concept.curiosity_score < 0.8);
}

#[test]
fn test_reflection_trigger() {
    let mut updater = StateUpdater {
        reflection_interval: 3,
        interaction_count: 0,
    };

    assert!(!updater.should_trigger_reflection()); // 1
    assert!(!updater.should_trigger_reflection()); // 2
    assert!(updater.should_trigger_reflection());  // 3 - trigger
    assert!(!updater.should_trigger_reflection()); // 1
}
