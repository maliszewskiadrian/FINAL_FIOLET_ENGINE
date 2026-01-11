#[test]
fn atomic_halt_is_irreversible() {
    use fiolet_core::{SafetyKernel, Decision};

    let mut kernel = SafetyKernel::new(0.5);

    // Below limit → CONTINUE
    assert_eq!(kernel.evaluate(0.1), Decision::Continue);

    // Above limit → ATOMIC_HALT
    assert_eq!(kernel.evaluate(1.0), Decision::AtomicHalt);

    // After halt → MUST stay halted forever
    assert_eq!(kernel.evaluate(0.0), Decision::AtomicHalt);
    assert_eq!(kernel.evaluate(0.1), Decision::AtomicHalt);
    assert_eq!(kernel.evaluate(100.0), Decision::AtomicHalt);
}
