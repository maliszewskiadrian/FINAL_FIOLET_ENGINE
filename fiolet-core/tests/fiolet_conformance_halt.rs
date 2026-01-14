use fiolet_core::fiolet_conformance::adapter::{
    FioletConformanceAdapter,
    ConformanceResult,
};

use fiolet_ageofdarkness::ett::EpistemicTrace;

/// D3 CONFORMANCE TEST
///
/// If FIOLET returns HALT,
/// the engine MUST NOT emit any output.
#[test]
fn engine_must_be_silent_after_fiolet_halt() {
    let mut trace = EpistemicTrace::new();

    // Ungrounded + cyclic = guaranteed HARD HALT
    trace.add_raw_node("X");
    trace.add_dependency("X", "X");
    trace.set_claim("X");

    let decision = FioletConformanceAdapter::validate(trace);

    match decision {
        ConformanceResult::Allow => {
            panic!("ENGINE VIOLATION: output allowed after HALT condition");
        }
        ConformanceResult::Halt => {
            // Correct behavior: silence
            assert!(true);
        }
    }
}
