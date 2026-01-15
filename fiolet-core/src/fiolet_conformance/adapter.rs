/// Result of FIOLET conformance validation
#[derive(Debug, PartialEq)]
pub enum ConformanceResult {
    Allow,
    Halt,
}

/// FIOLET Conformance Adapter
///
/// TEST-ONLY module.
/// NEVER linked into production kernel.
///
/// Rules:
/// - full epistemic trace required
/// - HALT is terminal
pub struct FioletConformanceAdapter;

impl FioletConformanceAdapter {
    /// Dummy validate function for testing without AgeOfDarkness
    pub fn validate<T>(_trace: T) -> ConformanceResult {
        // Always return Allow by default
        ConformanceResult::Allow
    }
}
