```md
# FIOLET SAFETY STANDARD
## Normative Safety Contract v1.0

This document defines the **mandatory safety standard** for all implementations
of the FINAL FIOLET ENGINE safety kernel.

This is a **normative document**.
Any system claiming compliance with FIOLET **MUST** satisfy all requirements
defined herein.

---

## 1. Scope

This standard applies to:

- The `fiolet-core` safety kernel
- Any FFI, wrapper, runtime, or downstream system invoking it
- Any future reimplementation claiming FIOLET compatibility

This standard governs **safety only**, not performance or liveness.

---

## 2. Safety Model

The FIOLET safety model is:

- Deterministic
- Fail-closed
- Monotonic
- Irreversible

Safety is enforced by **state latching**, not by recovery.

---

## 3. Core Safety Invariants (MANDATORY)

The following invariants are **absolute** and **non-negotiable**.

### I1 — Monotonic Halt

Once the kernel enters the halted state, it **MUST remain halted forever**.

There exists **no execution path** that allows a return to a running state.

---

### I2 — Single State Transition

The only permitted state transition is:

```

Running → Halted

```

No other transitions are allowed.

---

### I3 — Halt Dominance

If the kernel is halted, **every evaluation MUST return `AtomicHalt`**,
independent of input.

---

### I4 — Threshold Trigger

If an evaluated deviation:

- exceeds the configured limit, **OR**
- is non-finite (NaN / ±∞)

the kernel **MUST immediately latch into the halted state**.

---

### I5 — No Hidden State

All safety-relevant state **MUST be explicit** and fully contained
within the safety kernel structure.

Hidden, implicit, global, or external safety state is forbidden.

---

### I6 — Fail-Closed Semantics

Any undefined, exceptional, or erroneous condition **MUST resolve to HALT**.

Fail-open behavior is strictly prohibited.

---

## 4. Panic and Fault Handling

In kernel / no_std mode:

- Any panic **MUST result in permanent halt**
- Recovery from panic is forbidden

This requirement ensures fail-closed behavior under all fault conditions.

---

## 5. ABI Stability Requirements

The public ABI:

- MUST be stable
- MUST use `repr(C)` for all exposed types
- MUST preserve semantic meaning of return values

Breaking ABI safety semantics is considered a **standard violation**.

---

## 6. Formal Specification

The authoritative formal specification of this standard is defined in:

```

SafetyKernel.tla

```

The TLA+ specification is the **source of truth** for all safety behavior.

Code is considered correct **only if it conforms to the formal model**.

---

## 7. Compliance Verification

Compliance with this standard is enforced by:

- Executable tests
- Invariant tests
- Continuous Integration (CI)

In particular:

> The monotonic halt invariant is enforced by executable tests
> (`atomic_halt_irreversible`).

Failure of these tests **invalidates compliance**.

---

## 8. Non-Compliance

Any implementation that violates **any** requirement of this document:

- MUST NOT be labeled as FIOLET-compliant
- MUST NOT claim safety guarantees
- MUST be considered unsafe by definition

---

## 9. Versioning

This document defines:

```

FIOLET SAFETY STANDARD — Version 1.0

```

Any future changes **MUST** be explicit, versioned, and justified.

---

## END OF STANDARD
```

Known limitation (v0.1.0-snapshot):
Normative CI may fail due to an unresolved import in the pinned
FIOLET-AgeOfDarkness commit fcc4ae1.
This does not affect kernel correctness.
