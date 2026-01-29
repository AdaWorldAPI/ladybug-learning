# 🐞 ladybug-learning

**Meta-AGI Learning Loop — Standalone Rust Implementation**

> The learning curve IS the knowledge.  
> Similar problems FEEL similar before you know WHY.  
> Capture the feeling, retrieve the solution.

## ✅ Tested & Working

```
running 7 tests
test core::tests::test_bind_unbind ... ok
test core::tests::test_deterministic ... ok
test nars::tests::test_deduction ... ok
test core::tests::test_similarity ... ok
test nars::tests::test_from_evidence ... ok
test nars::tests::test_revision ... ok
test tests::test_full_learning_loop ... ok

test result: ok. 7 passed; 0 failed
```

## Quick Start

```bash
cargo run --example learning_loop
```

## The 6-Phase Learning Loop

```
┌─────────────────────────────────────────────────────────────────┐
│   1. ENCOUNTER   → Log to blackboard                            │
│   2. STRUGGLE    → Capture attempt vectors (resonance)          │
│   3. BREAKTHROUGH→ Extract concept (high satisfaction qualia)   │
│   4. CONSOLIDATE → Ice-cake decisions (FLOW/HOLD/BLOCK)         │
│   5. APPLY       → Query resonance for "felt this before"       │
│   6. META-LEARN  → Track what patterns work                     │
└─────────────────────────────────────────────────────────────────┘
```

## Core Components

| Module | Purpose |
|--------|---------|
| `core` | 10K-bit Fingerprint, Hamming distance, XOR bind |
| `cognitive` | ThinkingStyle, GateState, CollapseDecision |
| `nars` | TruthValue (frequency, confidence), inference |
| `learning/moment` | Moment, Qualia, MomentBuilder |
| `learning/session` | LearningSession, SessionPhase |
| `learning/resonance` | ResonanceCapture, Mexican Hat |
| `learning/blackboard` | Blackboard (YAML/JSON export) |
| `learning/concept` | ConceptExtractor (CAM fingerprints) |

## API Example

```rust
use ladybug_learning::prelude::*;
use ladybug_learning::MetaAGI;

let mut agi = MetaAGI::new();

// Start session
let session = agi.start_session("task-id", "description");
session.encounter("Found something");
session.struggle("This is hard", 0.7, 0.5);
session.breakthrough("Eureka!", 0.9);

// Capture moments
let moments: Vec<_> = agi.session().unwrap().moments.iter().cloned().collect();
for moment in &moments {
    agi.capture_moment(moment);
}

// Query resonance
let query = Fingerprint::from_content("similar problem");
let similar = agi.find_similar(&query, 0.6, 10);

// Export
agi.sync_blackboard();
println!("{}", agi.handover_summary());
```

## Resonance in Action

Session 2 hits resonance with Session 1 **before knowing why**:

```
🚀 Session 2: Implement Sprints
   ⚡ RESONANCE HIT! Found 3 similar past moments
   💭 "I've felt this before..." (resonance: 0.501)
   ✓ Pattern recognition accelerated learning!
```

## Integration with ladybug-rs

This standalone version embeds core primitives. To integrate with the full [ladybug-rs](https://github.com/AdaWorldAPI/ladybug-rs) crate:

```toml
[dependencies]
ladybug = { path = "../ladybug-rs" }
```

## License

Apache 2.0

---

**🧠 The shape of figuring it out IS the intelligence.**

After 100K moments: AGI emerges from accumulated learning-how-to-learn.
