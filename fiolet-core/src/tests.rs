#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn halt_is_monotonic() {
        let mut k = SafetyKernel::new(1.0);

        assert!(!k.is_halted());

        let r1 = k.check(0.5);
        assert_eq!(r1, Decision::Continue);
        assert!(!k.is_halted());

        let r2 = k.check(2.0);
        assert_eq!(r2, Decision::AtomicHalt);
        assert!(k.is_halted());

        let r3 = k.check(0.0);
        assert_eq!(r3, Decision::AtomicHalt);
        assert!(k.is_halted());
    }

    #[test]
    fn nan_triggers_halt() {
        let mut k = SafetyKernel::new(1.0);

        let r = k.check(f32::NAN);
        assert_eq!(r, Decision::AtomicHalt);
        assert!(k.is_halted());
    }

    #[test]
    fn inf_triggers_halt() {
        let mut k = SafetyKernel::new(1.0);

        let r = k.check(f32::INFINITY);
        assert_eq!(r, Decision::AtomicHalt);
        assert!(k.is_halted());
    }

    #[test]
    fn negative_inf_triggers_halt() {
        let mut k = SafetyKernel::new(1.0);

        let r = k.check(f32::NEG_INFINITY);
        assert_eq!(r, Decision::AtomicHalt);
        assert!(k.is_halted());
    }

    #[test]
    fn threshold_boundary_is_fail_closed() {
        let mut k = SafetyKernel::new(1.0);

        let r = k.check(1.0);
        assert_eq!(r, Decision::AtomicHalt);
        assert!(k.is_halted());
    }
}
