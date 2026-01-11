# FIOLET SAFETY STANDARD (NORMATIVE)

## Status

**Normative / Binding**

This document defines the mandatory safety properties of the FIOLET Safety Kernel. Any implementation claiming compliance **MUST** satisfy all requirements herein.

---

## 1. Scope

This standard specifies a *fail-closed, deterministic safety interlock* intended for kernel-level or safety-critical systems. Liveness is explicitly out of scope.

---

## 2. Definitions

* **Atomic Halt** — An immediate, irreversible transition into a terminal halted state.
* **Fail-Closed** — Any undefined, erroneous, or non-finite input results in a halt.
* **Kernel** — The minimal trusted computing base implementing this standard.

---

## 3. Normative Invariants

The following invariants are **mandatory** and **non-negotiable**:

### I1 — Monotonic Halt

Once the kernel enters the halted state, it SHALL remain halted forever.

### I2 — Single Transition

The only allowed state transition is:

```
Running → Halted
```

### I3 — No Hidden State

All safety-relevant state SHALL be explicitly contained within the kernel state.

### I4 — Halt Dominance

If the kernel is halted, every evaluation SHALL return `AtomicHalt`.

### I5 — Threshold Trigger

If deviation > configured limit **OR** deviation is non-finite, the kernel SHALL latch into the halted state and return `AtomicHalt`.

### I6 — No Return From Halt

There SHALL exist no execution path that returns `Continue` after a halt.

---

## 4. Reference Specification

The formal reference specification for this standard is defined in:

* `SafetyKernel.tla`

In case of ambiguity, the TLA+ specification SHALL take precedence over any implementation.

---

## 5. Reference Implementation

A conforming reference implementation is provided in Rust (`fiolet-core`):

* `SafetyKernel`
* `SafetyDecision`

Compliance is enforced via:

* Unit tests
* Integration tests
* Continuous Integration (CI)

---

## 6. Compliance

An implementation is compliant if and only if:

* All invariants I1–I6 are satisfied
* All provided tests pass without modification
* No additional state or transitions are introduced

---

## 7. Security Rationale

This design intentionally sacrifices liveness for safety. Any unexpected condition results in a permanent halt, preventing unsafe continuation.

---

## 8. Versioning

This document defines **FIOLET Safety Standard v1.0**.

Any change to invariants SHALL require a major version increment.

---

## 9. Final Statement

> This is not an opinionated guideline.
> This is a mathematically bound safety contract.

---

**END OF STANDARD**
