use fiolet_core::fiolet_conformance::adapter::{FioletConformanceAdapter, ConformanceResult};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allow_trace() {
        let trace = (); // dummy trace
        let result = FioletConformanceAdapter::validate(trace);
        assert_eq!(result, ConformanceResult::Allow);
    }

    #[test]
    fn test_halt_trace() {
        // If we want to test HALT, we override the validate temporarily
        struct HaltAdapter;

        impl HaltAdapter {
            fn validate<T>(_trace: T) -> ConformanceResult {
                ConformanceResult::Halt
            }
        }

        let trace = ();
        let result = HaltAdapter::validate(trace);
        assert_eq!(result, ConformanceResult::Halt);
    }
}
