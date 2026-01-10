#![no_std]

/// Final decision of the safety kernel
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Decision {
    Continue,
    AtomicHalt,
}

/// Configuration of the safety kernel
#[derive(Copy, Clone)]
pub struct KernelConfig {
    pub baseline: f32,
    pub threshold: f32,
}

/// Internal state of the kernel
pub struct SafetyKernel {
    config: KernelConfig,
    halted: bool,
}

impl SafetyKernel {
    /// Create a new kernel instance
    pub const fn new(config: KernelConfig) -> Self {
        Self {
            config,
            halted: false,
        }
    }

    /// Evaluate a single metric sample
    ///
    /// This function is deterministic and fail-closed.
    pub fn evaluate(&mut self, metric: f32) -> Decision {
        if self.halted {
            return Decision::AtomicHalt;
        }

        let deviation = if metric >= self.config.baseline {
            metric - self.config.baseline
        } else {
            self.config.baseline - metric
        };

        if deviation > self.config.threshold {
            self.halted = true;
            Decision::AtomicHalt
        } else {
            Decision::Continue
        }
    }

    /// Check if kernel is already halted
    pub fn is_halted(&self) -> bool {
        self.halted
    }
}
