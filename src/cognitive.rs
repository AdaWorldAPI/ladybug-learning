//! Cognitive primitives - embedded for standalone operation

use std::fmt;

/// Thinking style
#[derive(Clone, Debug, Default)]
pub struct ThinkingStyle {
    pub analytical: f32,
    pub creative: f32,
    pub focused: f32,
    pub exploratory: f32,
}

impl ThinkingStyle {
    pub fn analytical() -> Self {
        Self { analytical: 1.0, creative: 0.2, focused: 0.8, exploratory: 0.2 }
    }
    
    pub fn creative() -> Self {
        Self { analytical: 0.3, creative: 1.0, focused: 0.3, exploratory: 0.8 }
    }
    
    pub fn focused() -> Self {
        Self { analytical: 0.7, creative: 0.2, focused: 1.0, exploratory: 0.1 }
    }
    
    pub fn reflective() -> Self {
        Self { analytical: 0.6, creative: 0.5, focused: 0.5, exploratory: 0.6 }
    }
}

/// Collapse gate state
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GateState {
    Flow,
    Hold,
    Block,
}

impl fmt::Display for GateState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Flow => write!(f, "🟢 FLOW"),
            Self::Hold => write!(f, "🟡 HOLD"),
            Self::Block => write!(f, "🔴 BLOCK"),
        }
    }
}

/// Constants
pub const SD_MAX: f32 = 0.5;
pub const SD_FLOW_THRESHOLD: f32 = 0.30 * SD_MAX;
pub const SD_BLOCK_THRESHOLD: f32 = 0.70 * SD_MAX;

/// Calculate standard deviation
pub fn calculate_sd(values: &[f32]) -> f32 {
    if values.len() <= 1 {
        return 0.0;
    }
    let n = values.len() as f32;
    let mean = values.iter().sum::<f32>() / n;
    let variance = values.iter()
        .map(|&x| (x - mean) * (x - mean))
        .sum::<f32>() / n;
    variance.sqrt()
}

/// Get gate state from SD
pub fn get_gate_state(sd: f32) -> GateState {
    if sd < SD_FLOW_THRESHOLD {
        GateState::Flow
    } else if sd > SD_BLOCK_THRESHOLD {
        GateState::Block
    } else {
        GateState::Hold
    }
}

/// Collapse action
#[derive(Clone, Debug)]
pub enum CollapseAction {
    Collapse { winner_index: usize },
    Hold { sppm_key: String },
    Clarify { question: String },
    Block { reason: String },
}

/// Collapse decision
#[derive(Clone, Debug)]
pub struct CollapseDecision {
    pub state: GateState,
    pub sd: f32,
    pub can_collapse: bool,
    pub action: CollapseAction,
    pub reason: String,
    pub winner_index: Option<usize>,
    pub winner_score: Option<f32>,
}

/// Evaluate collapse gate
pub fn evaluate_gate(candidate_scores: &[f32], clarification_available: bool) -> CollapseDecision {
    if candidate_scores.is_empty() {
        return CollapseDecision {
            state: GateState::Block,
            sd: f32::INFINITY,
            can_collapse: false,
            action: CollapseAction::Block { reason: "No candidates".to_string() },
            reason: "Empty candidate set".to_string(),
            winner_index: None,
            winner_score: None,
        };
    }
    
    if candidate_scores.len() == 1 {
        return CollapseDecision {
            state: GateState::Flow,
            sd: 0.0,
            can_collapse: true,
            action: CollapseAction::Collapse { winner_index: 0 },
            reason: "Single candidate".to_string(),
            winner_index: Some(0),
            winner_score: Some(candidate_scores[0]),
        };
    }
    
    let sd = calculate_sd(candidate_scores);
    let state = get_gate_state(sd);
    
    let (winner_idx, winner_score) = candidate_scores.iter()
        .enumerate()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
        .map(|(i, &s)| (i, s))
        .unwrap_or((0, 0.0));
    
    match state {
        GateState::Flow => CollapseDecision {
            state: GateState::Flow,
            sd,
            can_collapse: true,
            action: CollapseAction::Collapse { winner_index: winner_idx },
            reason: format!("Low dispersion (SD={:.3})", sd),
            winner_index: Some(winner_idx),
            winner_score: Some(winner_score),
        },
        GateState::Hold => CollapseDecision {
            state: GateState::Hold,
            sd,
            can_collapse: false,
            action: CollapseAction::Hold { sppm_key: format!("sppm_{:x}", rand_u64()) },
            reason: format!("Medium dispersion (SD={:.3})", sd),
            winner_index: Some(winner_idx),
            winner_score: Some(winner_score),
        },
        GateState::Block => {
            if clarification_available {
                CollapseDecision {
                    state: GateState::Block,
                    sd,
                    can_collapse: false,
                    action: CollapseAction::Clarify {
                        question: "Multiple interpretations possible".to_string()
                    },
                    reason: format!("High dispersion (SD={:.3})", sd),
                    winner_index: Some(winner_idx),
                    winner_score: Some(winner_score),
                }
            } else {
                CollapseDecision {
                    state: GateState::Block,
                    sd,
                    can_collapse: false,
                    action: CollapseAction::Hold { sppm_key: format!("sppm_{:x}", rand_u64()) },
                    reason: format!("High dispersion, holding (SD={:.3})", sd),
                    winner_index: Some(winner_idx),
                    winner_score: Some(winner_score),
                }
            }
        }
    }
}

fn rand_u64() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

/// Layer ID for 7-layer consciousness
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LayerId {
    L1, L2, L3, L4, L5, L6, L7,
}

impl LayerId {
    pub const ALL: [LayerId; 7] = [
        Self::L1, Self::L2, Self::L3, Self::L4, Self::L5, Self::L6, Self::L7
    ];
    
    pub fn name(&self) -> &'static str {
        match self {
            Self::L1 => "Sensory",
            Self::L2 => "Pattern",
            Self::L3 => "Semantic",
            Self::L4 => "Episodic",
            Self::L5 => "Working",
            Self::L6 => "Executive",
            Self::L7 => "Meta",
        }
    }
    
    pub fn index(&self) -> usize {
        match self {
            Self::L1 => 0, Self::L2 => 1, Self::L3 => 2, Self::L4 => 3,
            Self::L5 => 4, Self::L6 => 5, Self::L7 => 6,
        }
    }
}
