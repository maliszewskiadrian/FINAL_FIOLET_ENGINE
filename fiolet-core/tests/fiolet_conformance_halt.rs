#[cfg(test)]
mod tests {
    use fiolet_core::fiolet_conformance::adapter::FioletConformanceAdapter;
    use fiolet_ageofdarkness::esv::EpistemicTrace;
    use fiolet_ageofdarkness::engine::ETTState;

    #[test]
    fn test_halt_trace() {
        let trace = EpistemicTrace::halt_example(); // przykładowa funkcja z AgeOfDarkness
        let result = FioletConformanceAdapter::validate(trace);
        assert_eq!(result, FioletConformanceAdapter::ConformanceResult::Halt);
    }
}
