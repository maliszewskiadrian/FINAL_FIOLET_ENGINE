//! TEST-ONLY module.
//! NEVER linked into production kernel.

use fiolet_ageofdarkness::engine::evaluate_trace;
use fiolet_ageofdarkness::esv::EpistemicTrace;

/// Result of FIOLET conformance validation
#[derive(Debug, PartialEq)]
pub enum ConformanceResult {
    Allow,
    Halt,
}

/// FIOLET Conformance Adapter
pub struct FioletConformanceAdapter;

impl FioletConformanceAdapter {
    pub fn validate(trace: EpistemicTrace) -> ConformanceResult {
        let result = evaluate_trace(trace);

        if result.is_halt() {
            ConformanceResult::Halt
        } else {
            ConformanceResult::Allow
        }
    }
}
