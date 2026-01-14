use fiolet_ageofdarkness::ett::*;
use fiolet_ageofdarkness::esv::*;

/// Result of FIOLET conformance validation
#[derive(Debug, PartialEq)]
pub enum ConformanceResult {
    Allow,
    Halt,
}

/// FIOLET Conformance Adapter
///
/// This adapter is the ONLY permitted integration point
/// between FINAL_FIOLET_ENGINE and the FIOLET standard.
///
/// NON-NEGOTIABLE:
/// - full epistemic trace required
/// - no partial emission
/// - HALT is terminal
pub struct FioletConformanceAdapter;

impl FioletConformanceAdapter {
    /// Validate full epistemic trace before emission.
    ///
    /// Silence on HALT is mandatory.
    pub fn validate(trace: EpistemicTrace) -> ConformanceResult {
        let mut ctx = ETTContext::from_trace(trace);
        let result = ctx.evaluate();

        if result.is_halt() {
            ConformanceResult::Halt
        } else {
            ConformanceResult::Allow
        }
    }
}
