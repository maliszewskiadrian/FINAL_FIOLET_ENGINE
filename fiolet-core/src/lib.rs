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
// Properties:
// - no_std
// - no alloc
// - no side effects
// - deterministic
// - fail-closed
// - FFI-safe
// - auditable
//
// ============================================================

use core::panic::PanicInfo;

// ============================================================
// PANIC HANDLER (REQUIRED FOR no_std)
// ============================================================

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Fail-closed by design:
    // any panic results in a permanent halt.
    loop {}
}

// ============================================================
// CORE TYPES (ABI STABLE)
// ============================================================

/// Core decision returned by the safety kernel.
///
/// repr(C) is REQUIRED for FFI correctness.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum SafetyDecision {
    /// Generation may continue.
    Continue = 0,
    /// Immediate and irreversible stop.
    AtomicHalt = 1,
}

/// Immutable safety threshold configuration.
///
/// Plain data only.
#[repr(C)]
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
/// Minimal, deterministic, monotonic.
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
    /// Guarantees:
    /// - deterministic
    /// - fail-closed
    /// - irreversible halt
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
