//! Core primitives - embedded for standalone operation

use std::hash::{Hash, Hasher};
use std::fmt;

/// Fingerprint dimensions
pub const FINGERPRINT_BITS: usize = 10_000;
pub const FINGERPRINT_U64: usize = 157;  // ceil(10000/64)

/// 10,000-bit VSA fingerprint for resonance operations
#[repr(align(64))]
#[derive(Clone)]
pub struct Fingerprint {
    data: [u64; FINGERPRINT_U64],
}

impl Fingerprint {
    pub fn from_raw(data: [u64; FINGERPRINT_U64]) -> Self {
        Self { data }
    }
    
    /// Create from content string (deterministic)
    pub fn from_content(content: &str) -> Self {
        use std::collections::hash_map::DefaultHasher;
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        let mut state = hasher.finish();
        
        let mut data = [0u64; FINGERPRINT_U64];
        for word in &mut data {
            let mut val = 0u64;
            for bit in 0..64 {
                let feedback = (state ^ (state >> 2) ^ (state >> 3) ^ (state >> 63)) & 1;
                state = (state >> 1) | (feedback << 63);
                val |= (state & 1) << bit;
            }
            *word = val;
        }
        
        Self { data }
    }
    
    pub fn random() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        Self::from_content(&format!("random_{}", seed))
    }
    
    pub fn zero() -> Self {
        Self { data: [0u64; FINGERPRINT_U64] }
    }
    
    pub fn as_raw(&self) -> &[u64; FINGERPRINT_U64] {
        &self.data
    }
    
    pub fn popcount(&self) -> u32 {
        self.data.iter().map(|x| x.count_ones()).sum()
    }
    
    #[inline]
    pub fn get_bit(&self, pos: usize) -> bool {
        let word = pos / 64;
        let bit = pos % 64;
        (self.data[word] >> bit) & 1 == 1
    }
    
    #[inline]
    pub fn set_bit(&mut self, pos: usize, value: bool) {
        let word = pos / 64;
        let bit = pos % 64;
        if value {
            self.data[word] |= 1 << bit;
        } else {
            self.data[word] &= !(1 << bit);
        }
    }
    
    /// Hamming distance
    #[inline]
    pub fn hamming(&self, other: &Fingerprint) -> u32 {
        self.data.iter()
            .zip(other.data.iter())
            .map(|(a, b)| (a ^ b).count_ones())
            .sum()
    }
    
    /// Similarity (0.0 - 1.0)
    #[inline]
    pub fn similarity(&self, other: &Fingerprint) -> f32 {
        1.0 - (self.hamming(other) as f32 / FINGERPRINT_BITS as f32)
    }
    
    /// XOR bind
    pub fn bind(&self, other: &Fingerprint) -> Fingerprint {
        let mut result = [0u64; FINGERPRINT_U64];
        for i in 0..FINGERPRINT_U64 {
            result[i] = self.data[i] ^ other.data[i];
        }
        Fingerprint { data: result }
    }
    
    #[inline]
    pub fn unbind(&self, other: &Fingerprint) -> Fingerprint {
        self.bind(other)
    }
    
    /// Permute (rotate bits)
    pub fn permute(&self, positions: i32) -> Fingerprint {
        let mut result = Self::zero();
        let total_bits = FINGERPRINT_BITS;
        let shift = positions.rem_euclid(total_bits as i32) as usize;
        
        for i in 0..total_bits {
            let new_pos = (i + shift) % total_bits;
            if self.get_bit(i) {
                result.set_bit(new_pos, true);
            }
        }
        result
    }
}

impl PartialEq for Fingerprint {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Eq for Fingerprint {}

impl Hash for Fingerprint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl fmt::Debug for Fingerprint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fingerprint({} bits set)", self.popcount())
    }
}

impl Default for Fingerprint {
    fn default() -> Self {
        Self::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_deterministic() {
        let fp1 = Fingerprint::from_content("hello");
        let fp2 = Fingerprint::from_content("hello");
        assert_eq!(fp1, fp2);
    }
    
    #[test]
    fn test_similarity() {
        let fp1 = Fingerprint::from_content("hello");
        let fp2 = Fingerprint::from_content("hello");
        assert_eq!(fp1.similarity(&fp2), 1.0);
        
        let fp3 = Fingerprint::from_content("world");
        let sim = fp1.similarity(&fp3);
        assert!(sim > 0.0 && sim < 1.0);
    }
    
    #[test]
    fn test_bind_unbind() {
        let a = Fingerprint::from_content("red");
        let b = Fingerprint::from_content("apple");
        let bound = a.bind(&b);
        let recovered = bound.unbind(&a);
        assert_eq!(recovered, b);
    }
}
