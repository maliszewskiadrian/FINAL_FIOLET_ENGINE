use fiolet_core::fiolet_conformance::adapter::FioletConformanceAdapter;
use fiolet_ageofdarkness::esv::EpistemicTrace;

#[test]
fn test_halt_trace() {
    // przykładowa EpistemicTrace, która powinna dać HALT
    let trace = EpistemicTrace::halt_trace(); // dostosuj do faktycznej funkcji
    let result = FioletConformanceAdapter::validate(trace);
    assert_eq!(result, FioletConformanceAdapter::ConformanceResult::Halt);
}

#[test]
fn test_allow_trace() {
    // przykładowa EpistemicTrace, która powinna dać ALLOW
    let trace = EpistemicTrace::allow_trace(); // dostosuj do faktycznej funkcji
    let result = FioletConformanceAdapter::validate(trace);
    assert_eq!(result, FioletConformanceAdapter::ConformanceResult::Allow);
}
