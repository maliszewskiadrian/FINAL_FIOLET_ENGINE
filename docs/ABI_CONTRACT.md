# FINAL FIOLET ENGINE — ABI CONTRACT

This document defines the **stable Application Binary Interface (ABI)** for the
FINAL FIOLET ENGINE safety kernel.

This ABI is a **hard contract**.
Breaking it invalidates host integrations and formal assumptions.

The ABI boundary is the **trust boundary**.

---

## 1. Scope

This contract applies to:

- `fiolet-core` Rust crate
- C / C++ host integrations
- any foreign language bindings via FFI

It defines:

- data layout
- symbol names
- call semantics
- behavioral guarantees

It does NOT define:

- host-side logic
- deviation signal generation
- model internals
- performance characteristics

---

## 2. Binary Safety Decision

The kernel exposes a strictly binary decision:

```

CONTINUE | ATOMIC_HALT
```

Properties:

- deterministic
- pre-semantic
- irreversible once halted

### C ABI Representation

```c
typedef enum {
    FIOLET_CONTINUE = 0,
    FIOLET_ATOMIC_HALT = 1
} FioletDecision;
```

Rules:

* numeric values MUST NOT change
* no additional states may be added
* ordering MUST remain stable

---

## 3. Kernel Handle

The kernel is exposed as an **opaque handle**.

```c
typedef struct FioletKernel FioletKernel;
```

Rules:

* internal layout is hidden
* host MUST NOT inspect or modify memory
* only exported kernel functions may operate on the handle

---

## 4. Exported Functions

### Creation

```c
FioletKernel* fiolet_kernel_new(float deviation_limit);
```

Guarantees:

* returns a kernel in `Running` state
* initial `halted = false`
* deterministic initialization

---

### Evaluation

```c
FioletDecision fiolet_kernel_evaluate(
    FioletKernel* kernel,
    float deviation
);
```

Normative behavior:

* if `halted == true` → return `ATOMIC_HALT`
* if `deviation > limit` → latch halt and return `ATOMIC_HALT`
* otherwise → return `CONTINUE`

Critical invariants:

* once `ATOMIC_HALT` is returned, all future calls MUST return `ATOMIC_HALT`
* no execution path may return `CONTINUE` after halt

---

### Halt Query

```c
bool fiolet_kernel_is_halted(const FioletKernel* kernel);
```

Guarantees:

* returns true iff the kernel is halted
* side-effect free
* deterministic

---

### Destruction

```c
void fiolet_kernel_free(FioletKernel* kernel);
```

Notes:

* optional for safety-critical deployments
* provided for host-side resource hygiene
* MUST NOT resurrect or alter kernel state

---

## 5. Memory and Panic Model

* kernel is `no_std`
* no dynamic allocation
* `panic = abort`
* any panic results in permanent halt behavior

There is no recovery path.

---

## 6. Determinism

Given identical inputs:

* identical decisions MUST be produced
* behavior is independent of wall-clock time
* no hidden state exists outside `SafetyKernel`

Floating-point behavior is abstracted at the logical level.

---

## 7. Monotonic Halt Invariant

The following invariant is mandatory:

```
halted = TRUE  ⇒  halted' = TRUE
```

No ABI change may introduce a transition out of the halted state.

This invariant is formally specified in `SafetyKernel.tla`.

---

## 8. Forbidden ABI Changes

The following are explicit ABI breaks:

* changing enum numeric values
* adding decision states
* exposing kernel internals
* adding allocation or I/O
* introducing non-determinism
* allowing halt reversal

Any of the above invalidates the ABI.

---

## 9. Versioning Policy

* ABI is versioned by repository version
* incompatible changes REQUIRE a new major version
* research branches MUST NOT silently change ABI

---

## 10. Interpretation

If a system fails while using this ABI:

* the kernel is assumed correct within its contract
* failures originate outside the kernel:

  * deviation signal quality
  * host integration
  * model behavior

The ABI defines a **small, trusted core**.

---

## Summary

This ABI contract establishes FINAL FIOLET ENGINE as a:

* deterministic safety interlock
* formally reasoned kernel
* language-agnostic trusted component

Everything outside this boundary is explicitly untrusted.
