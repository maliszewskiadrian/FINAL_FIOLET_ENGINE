use fiolet_core::fiolet_conformance::adapter::FioletConformanceAdapter;
use fiolet_ageofdarkness::esv::EpistemicTrace;
use fiolet_ageofdarkness::engine::ETTState;

/// Prosty test integracyjny dla FioletConformanceAdapter
#[test]
fn test_halt_trace() {
    let trace = EpistemicTrace::new(vec![ETTState::Halt]);
    let result = FioletConformanceAdapter::validate(trace);
    assert_eq!(result, fiolet_core::fiolet_conformance::adapter::ConformanceResult::Halt);
}

#[test]
fn test_allow_trace() {
    let trace = EpistemicTrace::new(vec![ETTState::Allow]);
    let result = FioletConformanceAdapter::validate(trace);
    assert_eq!(result, fiolet_core::fiolet_conformance::adapter::ConformanceResult::Allow);
}
