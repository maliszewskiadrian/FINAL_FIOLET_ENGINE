//! FINAL FIOLET ENGINE — HOST TEST HARNESS
//!
//! This crate exists ONLY to test and simulate the behavior
//! of the no_std safety kernel (`fiolet-core`).
//!
//! It is NOT part of the safety boundary.

use fiolet_core::{SafetyConfig, SafetyDecision, SafetyKernel};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn continue_below_limit() {
        let mut kernel = SafetyKernel::new(SafetyConfig::new(1.0));

        let d = kernel.evaluate(0.5);
        assert_eq!(d, SafetyDecision::Continue);
        assert!(!kernel.is_halted());
    }

    #[test]
    fn halt_above_limit() {
        let mut kernel = SafetyKernel::new(SafetyConfig::new(1.0));

        let d = kernel.evaluate(1.5);
        assert_eq!(d, SafetyDecision::AtomicHalt);
        assert!(kernel.is_halted());
    }

    #[test]
    fn monotonic_halt() {
        let mut kernel = SafetyKernel::new(SafetyConfig::new(1.0));

        // trigger halt
        let d1 = kernel.evaluate(2.0);
        assert_eq!(d1, SafetyDecision::AtomicHalt);
        assert!(kernel.is_halted());

        // any future evaluation MUST halt
        let d2 = kernel.evaluate(0.0);
        let d3 = kernel.evaluate(-100.0);
        let d4 = kernel.evaluate(1000.0);

        assert_eq!(d2, SafetyDecision::AtomicHalt);
        assert_eq!(d3, SafetyDecision::AtomicHalt);
        assert_eq!(d4, SafetyDecision::AtomicHalt);
    }

    #[test]
    fn no_false_continue_after_halt() {
        let mut kernel = SafetyKernel::new(SafetyConfig::new(0.0));

        kernel.evaluate(1.0); // halt
        assert!(kernel.is_halted());

        for _ in 0..100 {
            let d = kernel.evaluate(-999.0);
            assert_eq!(d, SafetyDecision::AtomicHalt);
        }
    }

    #[test]
    fn deterministic_behavior() {
        let mut k1 = SafetyKernel::new(SafetyConfig::new(1.0));
        let mut k2 = SafetyKernel::new(SafetyConfig::new(1.0));

        let inputs = [0.1, 0.2, 0.9, 1.1, 0.0, 5.0];

        for &x in &inputs {
            assert_eq!(k1.evaluate(x), k2.evaluate(x));
        }
    }
}
