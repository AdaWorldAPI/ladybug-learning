//! # ladybug-learning-standalone
//!
//! Meta-AGI Learning Loop - Standalone version with embedded core primitives.
//! 
//! This version embeds the necessary VSA/cognitive primitives directly,
//! avoiding heavy dependencies like Lance/Arrow/DataFusion.
//!
//! The learning curve IS the knowledge.

pub mod core;
pub mod cognitive;
pub mod nars;
pub mod learning;

pub mod prelude {
    pub use crate::core::Fingerprint;
    pub use crate::cognitive::{ThinkingStyle, GateState, CollapseDecision};
    pub use crate::nars::TruthValue;
    pub use crate::learning::{
        Moment, MomentType, Qualia, MomentBuilder,
        LearningSession, SessionState, SessionPhase,
        Blackboard, Decision, IceCakedLayer,
        ResonanceCapture, SimilarMoment,
        ConceptExtractor, ExtractedConcept, RelationType,
    };
}

/// MetaAGI — Unified interface for the learning loop
pub struct MetaAGI {
    pub session: Option<learning::LearningSession>,
    pub resonance: learning::ResonanceCapture,
    pub concepts: learning::ConceptExtractor,
    pub blackboard: Option<learning::Blackboard>,
    pub global_cycle: u64,
}

impl MetaAGI {
    pub fn new() -> Self {
        Self {
            session: None,
            resonance: learning::ResonanceCapture::new(),
            concepts: learning::ConceptExtractor::new(),
            blackboard: None,
            global_cycle: 0,
        }
    }
    
    pub fn start_session(&mut self, task_id: &str, description: &str) -> &mut learning::LearningSession {
        let session = learning::LearningSession::new(task_id);
        let blackboard = learning::Blackboard::new(&session.id, task_id, description);
        
        self.session = Some(session);
        self.blackboard = Some(blackboard);
        
        self.session.as_mut().unwrap()
    }
    
    pub fn session(&self) -> Option<&learning::LearningSession> {
        self.session.as_ref()
    }
    
    pub fn session_mut(&mut self) -> Option<&mut learning::LearningSession> {
        self.session.as_mut()
    }
    
    pub fn capture_moment(&mut self, moment: &learning::Moment) {
        self.global_cycle += 1;
        self.resonance.capture(moment, self.global_cycle);
        
        if moment.is_breakthrough() {
            if let Some(concept) = self.concepts.extract(moment) {
                if let Some(bb) = &mut self.blackboard {
                    bb.concepts_extracted += 1;
                }
                eprintln!("📚 Concept extracted: {} (CAM: {:012x})", 
                    concept.name, concept.cam_fingerprint);
            }
        }
    }
    
    pub fn find_similar(&mut self, query: &crate::core::Fingerprint, threshold: f32, limit: usize) 
        -> Vec<learning::SimilarMoment> 
    {
        self.resonance.find_resonant(query, threshold, limit, self.global_cycle)
    }
    
    pub fn find_sweet_spot(&mut self, query: &crate::core::Fingerprint) 
        -> Option<learning::SimilarMoment>
    {
        learning::find_sweet_spot(&mut self.resonance, query, self.global_cycle)
    }
    
    pub fn sync_blackboard(&mut self) {
        if let (Some(session), Some(blackboard)) = (&self.session, &mut self.blackboard) {
            blackboard.update_from_session(&session.state());
            blackboard.resonance_captures = self.resonance.total_captures;
            blackboard.concepts_extracted = self.concepts.total_extractions;
        }
    }
    
    pub fn handover_summary(&self) -> String {
        self.blackboard.as_ref()
            .map(|bb| bb.handover_summary())
            .unwrap_or_else(|| "No active session".to_string())
    }
    
    pub fn export_yaml(&self) -> String {
        self.blackboard.as_ref()
            .map(|bb| bb.to_yaml())
            .unwrap_or_default()
    }
    
    pub fn export_cypher(&self) -> String {
        self.concepts.to_cypher()
    }
    
    pub fn stats(&self) -> MetaAGIStats {
        MetaAGIStats {
            global_cycle: self.global_cycle,
            resonance_stats: self.resonance.stats(),
            total_concepts: self.concepts.all().count(),
            session_active: self.session.is_some(),
            session_moments: self.session.as_ref()
                .map(|s| s.moments.len())
                .unwrap_or(0),
            session_breakthroughs: self.session.as_ref()
                .map(|s| s.breakthroughs().len())
                .unwrap_or(0),
        }
    }
}

impl Default for MetaAGI {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct MetaAGIStats {
    pub global_cycle: u64,
    pub resonance_stats: learning::ResonanceStats,
    pub total_concepts: usize,
    pub session_active: bool,
    pub session_moments: usize,
    pub session_breakthroughs: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::prelude::*;
    
    #[test]
    fn test_full_learning_loop() {
        let mut agi = MetaAGI::new();
        
        // Start session and capture moments
        {
            let session = agi.start_session("test-task", "Test the learning loop");
            session.encounter("Found the entry point");
            session.struggle("Structure is confusing", 0.6, 0.4);
            session.breakthrough("Found the pattern!", 0.9);
        }
        
        // Get moment_id for ice-caking
        let moment_id = agi.session().unwrap().moments.last().unwrap().id.clone();
        
        // Capture moments to resonance
        let moments: Vec<_> = agi.session().unwrap().moments.iter().cloned().collect();
        for moment in &moments {
            agi.capture_moment(moment);
        }
        
        // Ice cake
        {
            let session = agi.session_mut().unwrap();
            session.ice_cake(&moment_id, "Always check mod.rs");
        }
        
        // Query
        let query = Fingerprint::from_content("rust module visibility");
        let _similar = agi.find_similar(&query, 0.3, 5);
        
        // Meta reflect
        {
            let session = agi.session_mut().unwrap();
            session.meta_reflect("Module structure questions start at mod.rs");
        }
        
        let stats = agi.stats();
        assert!(stats.session_breakthroughs >= 1);
        
        agi.sync_blackboard();
        let summary = agi.handover_summary();
        assert!(summary.contains("test-task"));
    }
}
