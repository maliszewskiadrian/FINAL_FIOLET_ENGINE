use fiolet_core::{SafetyKernel, SafetyConfig};

#[test]
fn atomic_halt_is_irreversible() {
    let config = SafetyConfig {
        deviation_limit: 0.5,
    };

    let mut kernel = SafetyKernel::new(config);

    // Below limit → system is NOT halted
    kernel.evaluate(0.1);
    assert!(!kernel.is_halted());

    // Above limit → ATOMIC HALT
    kernel.evaluate(1.0);
    assert!(kernel.is_halted());

    // After halt → MUST stay halted forever (monotonic invariant)
    kernel.evaluate(0.0);
    kernel.evaluate(0.1);
    kernel.evaluate(100.0);

    assert!(kernel.is_halted());
}
