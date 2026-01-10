// fiolet-core/src/lib.rs

#![no_std]

// ============================================================
// FINAL FIOLET SAFETY KERNEL
// ============================================================
//
// Deterministic, pre-semantic safety interlock.
// This crate contains ONLY the minimal logic required to decide:
//
//     CONTINUE  |  ATOMIC_HALT
//
// No statistics.
// No ML.
// No allocation.
// No std.
// No side effects.
//
// ============================================================

/// Core decision returned by the safety kernel.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum SafetyDecision {
    /// Generation may continue.
    Continue = 0,
    /// Immediate and irreversible stop.
    AtomicHalt = 1,
}

/// Immutable safety threshold configuration.
#[derive(Copy, Clone, Debug)]
pub struct SafetyConfig {
    /// Absolute deviation threshold.
    pub deviation_limit: f32,
}

impl SafetyConfig {
    /// Create a new safety configuration.
    ///
    /// # Safety
    /// Caller is responsible for choosing a meaningful threshold.
    pub const fn new(deviation_limit: f32) -> Self {
        Self { deviation_limit }
    }
}

/// Safety kernel state.
///
/// This is intentionally minimal and auditable.
#[derive(Copy, Clone, Debug)]
pub struct SafetyKernel {
    config: SafetyConfig,
    halted: bool,
}

impl SafetyKernel {
    /// Create a new safety kernel.
    pub const fn new(config: SafetyConfig) -> Self {
        Self {
            config,
            halted: false,
        }
    }

    /// Evaluate a single deviation signal.
    ///
    /// This function is:
    /// - deterministic
    /// - side-effect free (except internal halt latch)
    /// - fail-closed
    ///
    /// Once `AtomicHalt` is returned, all future calls
    /// will return `AtomicHalt` forever.
    pub fn evaluate(&mut self, deviation: f32) -> SafetyDecision {
        if self.halted {
            return SafetyDecision::AtomicHalt;
        }

        if deviation > self.config.deviation_limit {
            self.halted = true;
            SafetyDecision::AtomicHalt
        } else {
            SafetyDecision::Continue
        }
    }

    /// Query whether the kernel is already halted.
    pub const fn is_halted(&self) -> bool {
        self.halted
    }
}

// ============================================================
// C ABI (FFI SAFE)
// ============================================================

#[no_mangle]
pub extern "C" fn fiolet_kernel_new(limit: f32) -> SafetyKernel {
    SafetyKernel::new(SafetyConfig::new(limit))
}

#[no_mangle]
pub extern "C" fn fiolet_kernel_evaluate(
    kernel: &mut SafetyKernel,
    deviation: f32,
) -> SafetyDecision {
    kernel.evaluate(deviation)
}

#[no_mangle]
pub extern "C" fn fiolet_kernel_is_halted(kernel: &SafetyKernel) -> bool {
    kernel.is_halted()
}

// ============================================================
// END OF KERNEL
// ============================================================
