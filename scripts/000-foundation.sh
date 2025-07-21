#!/bin/bash

# STARWEAVE-MVP Foundation Script (Revised for Clarity)
# Creates a vector-based agent architecture with color-coded components

# --- Initialize Repository Structure ---
mkdir -p src/{concepts,embedding,actions,state}
mkdir -p benches tests/behavior examples docs/architecture
mkdir -p .github/workflows

# --- Create Core Rust Files ---
rm -f src/concepts.rs src/embedding.rs src/actions.rs 2>/dev/null

# Create concepts module (formerly harmonics)
cat > src/concepts/mod.rs << 'EOL'
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
EOL

# Create state module (formerly quantum)
cat > src/state/mod.rs << 'EOL'
// #7B68EE Stochastic State Management
use crate::concepts::ConceptVector;
use rand::Rng;

pub struct StateUpdater;

impl StateUpdater {
    pub fn new() -> Self {
        StateUpdater
    }

    // Applies a small random change to a vector's state
    pub fn update_state(&self, vector: &mut ConceptVector) {
        let mut rng = rand::thread_rng();
        vector.stochastic_state[0] += 0.01 * (rng.gen::<f32>() - 0.5);
        vector.stochastic_state[1] -= 0.01 * (rng.gen::<f32>() - 0.5);
    }
}
EOL

# Create embedding module (unchanged logic)
cat > src/embedding/mod.rs << 'EOL'
// #9400D3 Embedding Generator
use ndarray::Array1;
use anyhow::Result;

pub struct EmbeddingGenerator;

impl EmbeddingGenerator {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    pub fn embed(&self, text: &str) -> Result<Array1<f32>> {
        // Simple mock implementation for MVP
        let seed: f32 = text.len() as f32 / 100.0;
        let vec = vec![
            (0.5 + seed * 0.1).min(1.0),
            (-0.2 + seed * 0.05).max(-1.0),
            (0.4 - seed * 0.02).min(1.0)
        ];
        // Normalize the vector to prevent issues with cosine similarity
        let norm = vec.iter().map(|&x| x*x).sum::<f32>().sqrt();
        Ok(Array1::from_vec(vec.into_iter().map(|x| x/norm).collect()))
    }
}
EOL

# Create actions module (with fixed format strings)
cat > src/actions/mod.rs << 'EOL'
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
EOL

# Create main file with fixed println calls
cat > src/main.rs << 'EOL'
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
EOL

# --- Create Development Workflows ---
cat > .github/workflows/rust-ci.yml << 'EOL'
name: Rust CI

on: [push, pull_request]

jobs:
  build_and_test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install Rust Stable
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Cache dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/bin/
            ~/.cargo/registry/index/
            ~/.cargo/registry/cache/
            ~/.cargo/git/db/
            target/
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      - name: Build
        run: cargo build --release
      - name: Run tests
        run: cargo test
EOL

# --- Create Benchmark Script ---
cat > benches/similarity_benchmark.rs << 'EOL'
// #FF69B4 Similarity Performance Test
#![feature(test)]
extern crate test;

use starweave_mvp::concepts::{SimilarityEngine};
use ndarray::Array1;
use test::Bencher;

#[bench]
fn bench_similarity_search(b: &mut Bencher) {
    let engine = SimilarityEngine::new();
    let test_vector = Array1::from_vec(vec![0.85, -0.15, 0.45]);

    b.iter(|| {
        engine.find_best_match(&test_vector);
    });
}
EOL

# --- Create Dockerfile ---
cat > Dockerfile << 'EOL'
# #FFD700 Agent Deployment
FROM rust:1.78-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
# #9400D3 Minimal runtime
COPY --from=builder /app/target/release/starweave-mvp /usr/local/bin/starweave-mvp
CMD ["starweave-mvp"]
EOL

# --- Create Cargo Manifest (Updated with benchmark path) ---
cat > Cargo.toml << 'EOL'
[package]
name = "starweave-mvp"
version = "0.1.0"
edition = "2021"

[dependencies]
ndarray = { version = "0.15", features = ["serde"] }
serde = { version = "1.0", features = ["derive"] }
rand = "0.8"
anyhow = "1.0"

[[bench]]
name = "similarity_benchmark"
path = "benches/similarity_benchmark.rs"
harness = false
EOL

# --- Create .gitignore file ---
cat > .gitignore << 'EOL'
# Ignore Rust build artifacts
/target/
**/*.rs.bk

# Ignore Cargo lock file
Cargo.lock

# Ignore IDE-specific files
.vscode/
.idea/

# Ignore macOS system files
.DS_Store

# Ignore environment files
.env
EOL

# --- Create Development Script ---
mkdir -p scripts
cat > scripts/dev.sh << 'EOL'
#!/bin/bash
# #00CED1 Development Runner
cargo run --release
EOL
chmod +x scripts/dev.sh

# --- Create Architecture Documentation (Revised) ---
cat > docs/architecture/0001.md << 'EOL'
# STARWEAVE-MVP Architecture: Vector-Based Agent

## Core Components

### 🌈 Vector Similarity Engine (`#FF69B4`)
- **Purpose**: To find the best conceptual match between a user input and a set of predefined concepts.
- **Method**: Uses cosine similarity to measure the angular distance between high-dimensional vectors. A match is found if the similarity score exceeds a predefined threshold.
- **Technical Implementation**:
  ```rust
  pub fn find_best_match<'a>(&'a self, input_vec: &Array1<f32>) -> Option<&'a ConceptVector> {
      self.concepts.iter()
          .filter(|cv| cosine_similarity(&cv.vector, input_vec) > cv.threshold)
          .max_by(|a, b| ... )
  }
````

### ⚛️ Stochastic State Updater (`#7B68EE`)

  - **Purpose**: To introduce non-determinism into the system's state over time.
  - **Method**: Applies a minor, random modification to a concept's state vector after an interaction. This allows the system's behavior to drift and evolve subtly.
  - **Technical Implementation**:
    ```rust
    pub fn update_state(&self, vector: &mut ConceptVector) {
        vector.stochastic_state[0] += 0.01 * (rng.gen::<f32>() - 0.5);
        // ...
    }
    ```

### 🎨 Action System (`#00CED1`)

  - **Purpose**: To execute a specific function based on the best-matching concept.
  - **Method**: A `match` statement dispatches control to a specific action handler (e.g., `curiosity_action`) based on the `name` of the matched `ConceptVector`. Maintains a memory of recent inputs.
  - **Technical Implementation**:
    ```rust
    pub fn trigger_action(&mut self, concept: &ConceptVector, input: &str) -> String {
        match concept.name.as_str() {
            "Curiosity" => self.curiosity_action(input),
            // ...
        }
    }
    ```

## Key Capabilities

1.  **Intent-Based Action**: The system's behavior is driven by matching input to predefined concepts, rather than by direct commands.
2.  **Stochastic State**: A simple, randomized internal state allows for emergent, non-deterministic behavior over many interactions.
3.  **Stateful Memory**: The agent maintains a history of interactions, enabling context-aware responses in future versions.
4.  **Similarity Filtering**: The core engine effectively filters user input by ranking it against its internal concepts, prioritizing the most relevant response.

## Future Development Pathway

```mermaid
graph LR
    A[MVP: Mock Embeddings] --> B[v0.2: Real Sentence Embeddings]
    B --> C[v0.3: LLM-Generated Concepts]
    C --> D[v0.4: Multi-Agent Systems]
    D --> E[v1.0: Advanced Tool Use]
```

1.  **Phase 1 (Current MVP)**: Mock embeddings → Cosine similarity → CLI interface.
2.  **Phase 2 (v0.2)**: Integrate real ONNX sentence embeddings → Add a web interface → Implement shared memory across sessions.
3.  **Phase 3 (v0.3)**: Use an LLM to dynamically generate new `ConceptVector` instances → Improve state transition logic.
4.  **Phase 4 (v1.0)**: Explore interactions between multiple agents → Enable agents to use external tools/APIs based on intent.

## Ethical Considerations

  - **Agency Boundaries**: Maintain clear thresholds for autonomous action.
  - **Transparency**: Ensure similarity scores and triggered actions are auditable.
  - **Behavioral Safeguards**: Implement mechanisms to prevent undesirable feedback loops.
  - **Data Privacy**: Securely manage the agent's interaction memory.
EOL

#!/bin/bash

# --- Initialize Git Hooks (if .git exists) ---
if [ -d ".git" ]; then
    # Create hooks directory if missing
    mkdir -p .git/hooks

    # Create pre-commit hook with proper escaping
    cat > .git/hooks/pre-commit << 'EOL'
#!/bin/sh

# Run formatter and linter before committing

set -e
cargo fmt
cargo clippy -- -D warnings
cargo test
EOL
    chmod +x .git/hooks/pre-commit
fi

echo "🌟 STARWEAVE-MVP foundation established with revised, clearer terminology."
echo "🌈 Color-coded components:"
echo "  #FF69B4 - Vector Similarity (src/concepts)"
echo "  #9400D3 - Embedding Generator (src/embedding)"
echo "  #00CED1 - Autonomous Actions (src/actions)"
echo "  #7B68EE - Stochastic State (src/state)"
echo "  #FFD700 - System Manifest (src/main.rs)"
echo ""
echo "🚀 Next steps:"
echo "  1. Run: cargo build --release"
echo "  2. Execute: ./scripts/dev.sh"
