// fiolet-core/src/lib.rs

#![no_std]

// ============================================================
// FINAL FIOLET SAFETY KERNEL
// ============================================================
//
// Deterministic, pre-semantic safety interlock.
//
//     CONTINUE  |  ATOMIC_HALT
//
// No statistics.
// No ML.
// No allocation.
// No std.
// No side effects.
// Auditable.
// FFI-safe.
//
// ============================================================

use core::panic::PanicInfo;

// ============================================================
// PANIC HANDLER (REQUIRED FOR no_std)
// ============================================================

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Fail-closed: any panic = permanent halt
    loop {}
}

// ============================================================
// CORE TYPES
// ============================================================

/// Core decision returned by the safety kernel.
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum SafetyDecision {
    /// Generation may continue.
    Continue = 0,
    /// Immediate and irreversible stop.
    AtomicHalt = 1,
}

/// Immutable safety threshold configuration.
#[derive(Copy, Clone)]
pub struct SafetyConfig {
    /// Absolute deviation threshold.
    pub deviation_limit: f32,
}

impl SafetyConfig {
    /// Create a new safety configuration.
    pub const fn new(deviation_limit: f32) -> Self {
        Self { deviation_limit }
    }
}

/// Safety kernel state.
///
/// Minimal, deterministic, auditable.
#[repr(C)]
#[derive(Copy, Clone)]
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
    /// Properties:
    /// - deterministic
    /// - fail-closed
    /// - monotonic (halt is irreversible)
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

    /// Query whether the kernel is halted.
    pub const fn is_halted(&self) -> bool {
        self.halted
    }
}

// ============================================================
// C ABI (FFI SAFE INTERFACE)
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
// END OF FINAL FIOLET SAFETY KERNEL
// ============================================================
