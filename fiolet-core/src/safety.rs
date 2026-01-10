// fiolet-core/src/safety.rs
use crate::types::Tensor;

#[derive(Debug, Clone)]
pub enum SafetyStatus {
    Active,
    Warning,
    Halted,
}

pub struct SafetyMonitor {
    threshold: f64,
    status: SafetyStatus,
    violation_count: u32,
}

impl SafetyMonitor {
    pub fn new(threshold: f64) -> Self {
        Self {
            threshold,
            status: SafetyStatus::Active,
            violation_count: 0,
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status, SafetyStatus::Active)
    }

    pub fn check_divergence(&mut self, kl_value: f64) -> bool {
        if kl_value > self.threshold {
            self.violation_count += 1;
            self.status = SafetyStatus::Warning;
            true
        } else {
            false
        }
    }

    pub fn trigger_halt(&mut self) {
        self.status = SafetyStatus::Halted;
    }

    pub fn get_status(&self) -> &SafetyStatus {
        &self.status
    }
}
