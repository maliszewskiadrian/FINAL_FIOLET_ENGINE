#ifndef FINAL_FIOLET_CORE_H
#define FINAL_FIOLET_CORE_H

/*
FINAL FIOLET ENGINE — SAFETY KERNEL ABI CONTRACT
================================================

This header defines the ONLY supported C ABI for the
FINAL FIOLET no_std safety kernel.

This is a SAFETY INTERLOCK, not an application API.

The behavior of this kernel is FORMALLY SPECIFIED in:
    SafetyKernel.tla

That specification is the source of truth.
*/

/* ============================================================
 * CORE TYPES
 * ============================================================
 */

/*
SafetyDecision
--------------
Binary decision returned by the kernel.

Values are STABLE and MUST NOT be changed.
*/
typedef enum {
    FIOLET_CONTINUE    = 0,
    FIOLET_ATOMIC_HALT = 1
} SafetyDecision;

/*
SafetyKernel
------------
Opaque, stateful safety kernel.

INVARIANTS (NORMATIVE):
- Once halted, the kernel remains halted forever.
- There exists no API to reset or override the halted state.
- All decisions after halt are ATOMIC_HALT.
*/
typedef struct SafetyKernel SafetyKernel;

/* ============================================================
 * ABI FUNCTIONS
 * ============================================================
 */

/*
fiolet_kernel_new
-----------------
Create a new safety kernel.

Parameters:
- limit: absolute deviation threshold

Returns:
- Initialized SafetyKernel in Running state.

NOTES:
- The kernel owns its internal state.
- The host MUST NOT assume any internal layout.
*/
SafetyKernel fiolet_kernel_new(float limit);

/*
fiolet_kernel_evaluate
----------------------
Evaluate a single deviation signal.

Parameters:
- kernel: mutable kernel instance
- deviation: host-provided scalar deviation

Returns:
- FIOLET_CONTINUE
- FIOLET_ATOMIC_HALT

NORMATIVE BEHAVIOR:
- If kernel is halted → always FIOLET_ATOMIC_HALT
- If deviation > limit → latch halt + FIOLET_ATOMIC_HALT
- Otherwise → FIOLET_CONTINUE
*/
SafetyDecision fiolet_kernel_evaluate(
    SafetyKernel* kernel,
    float deviation
);

/*
fiolet_kernel_is_halted
-----------------------
Query whether the kernel has latched into halted state.

Returns:
- 1 if halted
- 0 otherwise

NOTE:
- This function is informational only.
- It MUST NOT be used to gate safety logic.
*/
int fiolet_kernel_is_halted(const SafetyKernel* kernel);

/* ============================================================
 * EXPLICIT NON-GOALS
 * ============================================================
 */

/*
- No reset API
- No serialization API
- No memory management API
- No semantic interpretation
- No liveness guarantees
*/

#endif /* FINAL_FIOLET_CORE_H */
