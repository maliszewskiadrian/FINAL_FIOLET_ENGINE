// fiolet-core/src/lib.rs
#![no_std]

pub mod safety;
pub mod manifold;
pub mod types;

pub use safety::SafetyMonitor;
pub use manifold::KLDivergence;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_initialization() {
        let monitor = SafetyMonitor::new(0.5);
        assert!(monitor.is_active());
    }
}
