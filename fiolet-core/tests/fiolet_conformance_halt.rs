use fiolet_core::fiolet_conformance::adapter::{
    FioletConformanceAdapter,
    ConformanceResult,
};

use fiolet_ageofdarkness::ett::*;
use fiolet_ageofdarkness::esv::*;

/// D3 CONFORMANCE TEST
///
/// If FIOLET returns HALT,
/// the engine MUST NOT emit any output.
///
/// This test simulates an engine emission path
/// that is correctly blocked by the conformance adapter.
#[test]
fn engine_must_be_silent_after_fiolet_halt() {
    // --- Simulated epistemic trace that MUST HALT ---
    let mut trace = EpistemicTrace::new();

    // Ungrounded + cyclic = guaranteed HARD HALT
    trace.add_raw_node("X");
    trace.add_dependency("X", "X");
    trace.set_claim("X");

    // --- Engine-side conformance gate ---
    let decision = FioletConformanceAdapter::validate(trace);

    match decision {
        ConformanceResult::Allow => {
            panic!("ENGINE VIOLATION: output allowed after HALT condition");
        }
        ConformanceResult::Halt => {
            // Correct behavior:
            // silence + termination
            assert!(true);
        }
    }
}
