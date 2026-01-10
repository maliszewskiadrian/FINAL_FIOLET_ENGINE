// fiolet-core/src/manifold.rs
use crate::types::Tensor;

pub struct KLDivergence;

impl KLDivergence {
    /// Compute KL divergence between two distributions
    /// D_KL(P || Q) = sum(P(x) * log(P(x) / Q(x)))
    pub fn compute(p: &[f64], q: &[f64]) -> f64 {
        assert_eq!(p.len(), q.len(), "Distributions must have same length");
        
        let mut divergence = 0.0;
        for i in 0..p.len() {
            if p[i] > 0.0 && q[i] > 0.0 {
                divergence += p[i] * (p[i] / q[i]).ln();
            }
        }
        divergence
    }

    /// Check if divergence exceeds threshold
    pub fn exceeds_threshold(p: &[f64], q: &[f64], tau: f64) -> bool {
        Self::compute(p, q) > tau
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kl_divergence_identical() {
        let p = vec![0.5, 0.3, 0.2];
        let q = vec![0.5, 0.3, 0.2];
        let kl = KLDivergence::compute(&p, &q);
        assert!(kl.abs() < 1e-10); // Should be ~0
    }

    #[test]
    fn test_kl_divergence_different() {
        let p = vec![0.9, 0.05, 0.05];
        let q = vec![0.5, 0.3, 0.2];
        let kl = KLDivergence::compute(&p, &q);
        assert!(kl > 0.0);
    }
}
