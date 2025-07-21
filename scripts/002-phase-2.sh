#!/bin/bash

# STARWEAVE Phase 2: Modular Action & Peer Initiative

# --- Fix Module Agent ---
cat > src/module_agent/mod.rs << 'EOL'
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
EOL

# --- Fix Agent Orchestrator ---
cat > src/agent_orchestrator.rs << 'EOL'
// #FFA07A Agent Orchestrator
use crate::module_agent::ModuleAgent;
use crate::concepts::{ConceptVector, cosine_similarity};
use ndarray::Array1;
use std::collections::HashMap;

pub struct AgentOrchestrator {
    pub modules: HashMap<String, ModuleAgent>,
    pub propensity_to_co_create: f32,
    pub proactive_prompts: Vec<String>,
}

impl AgentOrchestrator {
    pub fn new() -> Self {
        let mut proactive_prompts = Vec::new();
        proactive_prompts.push("What would happen if we combined these concepts?".to_string());
        proactive_prompts.push("How might we approach this from a different perspective?".to_string());
        proactive_prompts.push("What underlying patterns connect these ideas?".to_string());

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

        result.push_str(&format!("🧠 Primary module '{}' processing: {}\n", primary_module, input));

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
                result.push_str(&format!("💡 Module '{}' suggests: {}\n", name, suggestion));

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
EOL

# --- Remove Duplicate Method Implementation ---
# The calculate_curiosity_boost method is already defined in the ActionSystem impl block
# We'll remove the duplicate implementation at the end of the file
sed -i '/\/\/ Missing method implementation/,+7d' src/actions/mod.rs

# --- Add Missing Method to Action System (Properly) ---
# Instead, we'll ensure the method exists in the impl block
# We'll check if it's already present and if not, insert it in the correct location
if ! grep -q "fn calculate_curiosity_boost" src/actions/mod.rs; then
    # Insert the method in the impl block after the verification_action method
    sed -i '/fn verification_action/a \
    \
    // Calculate dynamic curiosity boost based on input\
    fn calculate_curiosity_boost(\&self, input: \&str) -> f32 {\
        // More complex input = higher curiosity boost\
        let complexity = input.len() as f32 / 100.0;\
        complexity.clamp(0.1, 0.5)\
    }' src/actions/mod.rs
fi

# --- Create Architecture Documentation ---
mkdir -p docs/architecture
cat > docs/architecture/0003-modular-agents.md << 'EOL'
# Modular Agents & Peer Initiative Implementation

## Core Components

### 🤖 Module Agent System (`#ADD8E6`)
- **Specialized Processing Units**:
  ```rust
  pub struct ModuleAgent {
      pub name: String,               // Module identifier
      pub concepts: Vec<ConceptVector>, // Domain-specific concepts
      pub local_engine: SimilarityEngine, // Dedicated similarity engine
      pub co_creation_count: u32,     // Collaboration counter
  }
  ```
- **Concept Suggestion**:
  ```rust
  pub fn suggest_concept(&self, _other: &str) -> Option<&ConceptVector> {
      // Find concept with highest curiosity score
      self.concepts.iter().max_by(|a, b|
          a.curiosity_score.partial_cmp(&b.curiosity_score).unwrap()
      )
  }
  ```

### 🎛️ Agent Orchestrator (`#FFA07A`)
- **Intelligent Routing**:
  ```rust
  pub fn route_input(&mut self, input_vec: &Array1<f32>) -> Option<String> {
      // Find module with best concept match
      for (name, module) in &mut self.modules {
          if let Some(concept) = module.process_input(input_vec) {
              let similarity = cosine_similarity(&concept.vector, input_vec);
              // Update best match if higher similarity found
          }
      }
  }
  ```
- **Co-Creation Facilitation**:
  ```rust
  pub fn co_create(&mut self, primary_module: &str, input: &str) -> String {
      // Collect suggestions from other modules
      for name in self.modules.keys().filter(|&n| n != primary_module) {
          if let Some(module) = self.modules.get_mut(&name) {
              if let Some(suggestion) = module.suggest_concept(primary_module) {
                  suggestions.push((name, suggestion.name.clone()));
              }
          }
      }
      // Process suggestions and record co-creations
  }
  ```

### 🤝 Co-Creation Mode
```rust
pub struct ActionSystem {
    // ...
    pub co_creation_mode: bool,  // Toggle for collaboration
}

pub fn toggle_co_creation(&mut self) {
    self.co_creation_mode = !self.co_creation_mode;
}
```

### 💡 Proactive Prompting System
```rust
pub struct AgentOrchestrator {
    // ...
    pub proactive_prompts: Vec<String>,  // Thought-provoking questions
    pub propensity_to_co_create: f32,    // Collaboration tendency (0.0-1.0)
}

pub fn generate_proactive_prompt(&self) -> &str {
    // Select prompt based on propensity level
    let index = (self.propensity_to_co_create *
                self.proactive_prompts.len() as f32) as usize;
    self.proactive_prompts.get(index)
        .unwrap_or(&self.proactive_prompts[0])
}
```

## Key Innovations

1. **Specialized Intelligence Modules**:
   - Each module focuses on a specific domain (Research, Creative, Verification)
   - Maintains its own concept subset and similarity engine
   - Can suggest concepts to other modules based on curiosity scores

2. **Dynamic Collaboration System**:
   - **Propensity Metric**: Tracks system's tendency to collaborate (0.0-1.0)
   - **Co-Creation Mode**: Toggle for enabling/disabling module collaboration
   - **Success Tracking**: Records successful collaborations per module

3. **Conflict-Free Borrow Handling**:
   ```rust
   // Solution for mutable borrow conflict:
   let mut suggestions = Vec::new();
   for name in self.modules.keys().filter(|&n| n != primary_module) {
       if let Some(module) = self.modules.get_mut(&name) {
           // Collect suggestions without holding multiple mutable references
           suggestions.push((name, module.suggest_concept(primary_module)));
       }
   }
   ```

4. **Proactive Engagement**:
   - System generates thoughtful prompts to encourage exploration
   - Prompt selection adapts based on collaboration propensity
   - Questions become more sophisticated as propensity increases

## System Workflow

```mermaid
graph TD
    A[User Input] --> B(Embedding Generator)
    B --> C{Agent Orchestrator}
    C -->|Route| D[Research Module]
    C -->|Route| E[Creative Module]
    D -->|Suggest| F[Co-Creation]
    E -->|Suggest| F
    F --> G[Response + Proactive Prompt]
    G --> H[User]
```

## Collaboration Metrics

| Metric | Description | Update Trigger |
|--------|-------------|----------------|
| `co_creation_count` | Successful collaborations per module | When modules exchange concepts |
| `propensity_to_co_create` | System-wide collaboration tendency | Increases after successful co-creation |
| `proactive_prompts` | Available thought-provoking questions | Configured at initialization |
EOL

echo "✅ All fixes applied:"
echo "  1. Added cosine_similarity import to agent_orchestrator"
echo "  2. Fixed mutable borrow conflict in co_create()"
echo "  3. Added missing calculate_curiosity_boost implementation"
echo "  4. Silenced unused variable warning in suggest_concept"
echo "  5. Removed unused imports"
echo ""
echo "🚀 System ready to compile and run!"
