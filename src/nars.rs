//! NARS primitives - embedded for standalone operation

use std::fmt;

/// NARS Truth Value (frequency, confidence)
#[derive(Clone, Debug)]
pub struct TruthValue {
    pub frequency: f32,
    pub confidence: f32,
}

impl TruthValue {
    pub fn new(frequency: f32, confidence: f32) -> Self {
        Self {
            frequency: frequency.clamp(0.0, 1.0),
            confidence: confidence.clamp(0.0, 1.0),
        }
    }
    
    /// Unknown truth value
    pub fn unknown() -> Self {
        Self { frequency: 0.5, confidence: 0.0 }
    }
    
    /// Certain true
    pub fn certain_true() -> Self {
        Self { frequency: 1.0, confidence: 0.9 }
    }
    
    /// Certain false
    pub fn certain_false() -> Self {
        Self { frequency: 0.0, confidence: 0.9 }
    }
    
    /// From positive/negative evidence counts
    pub fn from_evidence(positive: f32, negative: f32) -> Self {
        let total = positive + negative;
        if total == 0.0 {
            return Self::unknown();
        }
        let frequency = positive / total;
        let confidence = total / (total + 1.0); // k=1 horizon
        Self { frequency, confidence }
    }
    
    /// Expectation: E = c * (f - 0.5) + 0.5
    pub fn expectation(&self) -> f32 {
        self.confidence * (self.frequency - 0.5) + 0.5
    }
    
    /// Deduction: A→B, B→C ⊢ A→C
    pub fn deduction(&self, other: &TruthValue) -> TruthValue {
        let f = self.frequency * other.frequency;
        let c = self.confidence * other.confidence * self.frequency * other.frequency;
        TruthValue::new(f, c)
    }
    
    /// Induction: A→B, A→C ⊢ B→C
    pub fn induction(&self, other: &TruthValue) -> TruthValue {
        let f = other.frequency;
        let c = self.frequency * self.confidence * other.confidence / (self.frequency + 1.0);
        TruthValue::new(f, c)
    }
    
    /// Abduction: A→B, C→B ⊢ A→C
    pub fn abduction(&self, other: &TruthValue) -> TruthValue {
        let f = self.frequency;
        let c = other.frequency * self.confidence * other.confidence / (other.frequency + 1.0);
        TruthValue::new(f, c)
    }
    
    /// Revision: combine independent evidence
    pub fn revision(&self, other: &TruthValue) -> TruthValue {
        let w1 = self.confidence / (1.0 - self.confidence + f32::EPSILON);
        let w2 = other.confidence / (1.0 - other.confidence + f32::EPSILON);
        let w = w1 + w2;
        
        let f = (w1 * self.frequency + w2 * other.frequency) / (w + f32::EPSILON);
        let c = w / (w + 1.0);
        
        TruthValue::new(f, c)
    }
    
    /// Negation
    pub fn negation(&self) -> TruthValue {
        TruthValue::new(1.0 - self.frequency, self.confidence)
    }
}

impl Default for TruthValue {
    fn default() -> Self {
        Self::unknown()
    }
}

impl fmt::Display for TruthValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "⟨{:.0}%, {:.0}%⟩", self.frequency * 100.0, self.confidence * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_from_evidence() {
        let tv = TruthValue::from_evidence(9.0, 1.0);
        assert!((tv.frequency - 0.9).abs() < 0.01);
    }
    
    #[test]
    fn test_deduction() {
        let birds_fly = TruthValue::new(0.9, 0.9);
        let tweety_bird = TruthValue::certain_true();
        let result = birds_fly.deduction(&tweety_bird);
        assert!(result.frequency > 0.8);
    }
    
    #[test]
    fn test_revision() {
        let ev1 = TruthValue::new(0.8, 0.8);
        let ev2 = TruthValue::new(0.9, 0.7);
        let combined = ev1.revision(&ev2);
        // Combined should be between the two and higher confidence
        assert!(combined.frequency > 0.75 && combined.frequency < 0.95);
    }
}
