// fiolet-core/src/lib.rs

//! FINAL FIOLET ENGINE — SAFETY KERNEL
//!
//! NORMATIVE SAFETY CONTRACT
//! =========================
//!
//! This module implements a deterministic, fail-closed safety kernel.
//! Its behavior is FORMALLY SPECIFIED in the file:
//!
//! SafetyKernel.tla
//!
//! That specification is the SOURCE OF TRUTH.
//! Any change to logic MUST preserve all listed invariants.
//!
//! -------------------------
//! FORMAL INVARIANTS (BOUND)
//! -------------------------
//!
//! I1 — Monotonic Halt
//! Once the kernel enters the halted state, it MUST remain halted forever.
//!
//! I2 — Single Transition
//! The only allowed state transition is: Running → Halted.
//!
//! I3 — No Hidden State
//! All safety-relevant state is explicitly contained within `SafetyKernel`.
//!
//! I4 — Halt Dominance
//! If the kernel is halted, every evaluation MUST return `AtomicHalt`.
//!
//! I5 — Threshold Trigger
//! If deviation > limit OR deviation is non-finite,
//! the kernel MUST latch into halted state and return `AtomicHalt`.
//!
//! I6 — No Return From Halt
//! There exists no execution path that returns `Continue` after a halt.
//!
//! -------------------------
//! SYSTEM CONSTRAINTS
//! -------------------------
//!
//! - no_std (kernel mode)
//! - no allocation
//! - panic = abort (kernel mode)
//! - deterministic execution
//! - fail-closed by construction
//!
//! This is a SAFETY INTERLOCK, not an application.
//! Liveness is explicitly NOT guaranteed.

#![cfg_attr(not(any(test, feature = "std")), no_std)]

// ============================================================
// FINAL FIOLET SAFETY KERNEL
// ============================================================

#[cfg(not(any(test, feature = "std")))]
use core::panic::PanicInfo;

// ============================================================
// PANIC HANDLER (KERNEL ONLY — NEVER DURING TESTS)
// ============================================================

#[cfg(not(any(test, feature = "std")))]
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
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
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
    pub fn evaluate(&mut self, deviation: f32) -> SafetyDecision {
        if self.halted {
            return SafetyDecision::AtomicHalt;
        }

        if !deviation.is_finite() {
            self.halted = true;
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
// TESTS (STD ONLY — NOT PART OF TRUSTED KERNEL)
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn atomic_halt_is_irreversible() {
        let mut kernel = SafetyKernel::new(SafetyConfig::new(0.5));

        assert_eq!(kernel.evaluate(0.1), SafetyDecision::Continue);
        assert_eq!(kernel.evaluate(1.0), SafetyDecision::AtomicHalt);
        assert_eq!(kernel.evaluate(0.0), SafetyDecision::AtomicHalt);
    }
}

// ============================================================
// CONFORMANCE ADAPTER (ENGINE ⇄ STANDARD BOUNDARY)
// ============================================================

#[cfg(feature = "std")]
pub mod fiolet_conformance;

// ============================================================
// END OF FINAL FIOLET SAFETY KERNEL
// ============================================================
