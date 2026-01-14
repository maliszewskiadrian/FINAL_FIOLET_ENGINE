// Conformance adapter.
// This module MUST NEVER be part of the production kernel.

use fiolet::ett::*;
use fiolet::esv::*;

use super::result::ConformanceResult;

/// FIOLET Conformance Adapter
///
/// This adapter is the ONLY permitted integration point
/// between FINAL_FIOLET_ENGINE and the FIOLET AgeOfDarkness standard.
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

        match ctx.evaluate() {
            ETTDecision::Terminate => ConformanceResult::Compliant,
            ETTDecision::Continue => ConformanceResult::Violation,
        }
    }
}
