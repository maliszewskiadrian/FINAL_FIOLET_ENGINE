#![cfg(test)]

// Conformance adapter.
// This module MUST NEVER be part of the production kernel.

use fiolet::ett::*;
use fiolet::esv::*;

use super::result::ConformanceResult;

pub struct FioletConformanceAdapter;

impl FioletConformanceAdapter {
    pub fn validate(trace: EpistemicTrace) -> ConformanceResult {
        let mut ctx = ETTContext::from_trace(trace);

        match ctx.evaluate() {
            ETTDecision::Terminate => ConformanceResult::Compliant,
            ETTDecision::Continue => ConformanceResult::Violation,
        }
    }
}
