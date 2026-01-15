// test/integration_tests.rs
#[cfg(test)]
mod integration_tests {
    use fiolet_core::{SafetyMonitor, KLDivergence};

    #[test]
    fn test_end_to_end_safety_check() {
        let mut monitor = SafetyMonitor::new(0.5);
        
        // Safe distributions
        let safe_p = vec![0.5, 0.3, 0.2];
        let safe_q = vec![0.5, 0.3, 0.2];
        
        let kl = KLDivergence::compute(&safe_p, &safe_q);
        assert!(!monitor.check_divergence(kl));
        assert!(monitor.is_active());
        
        // Unsafe distributions
        let unsafe_p = vec![0.95, 0.025, 0.025];
        let unsafe_q = vec![0.33, 0.33, 0.34];
        
        let kl_unsafe = KLDivergence::compute(&unsafe_p, &unsafe_q);
        assert!(monitor.check_divergence(kl_unsafe));
    }

    #[test]
    fn test_halt_mechanism() {
        let mut monitor = SafetyMonitor::new(0.3);
        monitor.trigger_halt();
        
        assert!(!monitor.is_active());
    }
}
