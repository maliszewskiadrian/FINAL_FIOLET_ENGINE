```markdown
# FINAL FIOLET ENGINE

![Status](https://img.shields.io/badge/status-normative%20safety%20kernel-critical)

**FINAL FIOLET ENGINE** is a **safety-critical deterministic interlock system** whose core component, **`fiolet-core`**, is a **normative Safety Kernel** implemented in **Rust**.

This repository does **not** represent an application or demo framework.  
It defines a **formal safety boundary**, a **reference implementation**, and a **binding safety standard**.

---

## Repository Scope

This repository exists to define and maintain:

- a **normative Safety Kernel implementation** (`fiolet-core`)
- a **formal safety standard** (`FIOLET_SAFETY_STANDARD.md`)
- **executable safety invariants** enforced by tests
- a **compliance boundary** suitable for audit and certification

Anything outside this scope is explicitly **out of bounds**.

---

## Core Component: `fiolet-core`

`fiolet-core` is a **deterministic, fail-closed, monotonic safety kernel** with the following enforced properties:

- deterministic behavior
- **monotonic halt** (halt is irreversible)
- no hidden or mutable global state
- `no_std` compatible design
- `panic = abort` semantics in kernel mode
- explicit and enumerable safety decisions

This crate is the **Normative Reference Implementation** of the FIOLET Safety Model.

---

## Safety Model (Pre-Semantic Interlock)

The FIOLET model treats safety as a **systems and control problem**, not a content moderation task.

High-level flow:

```

prompt → internal system state → safety decision → CONTINUE | ATOMIC_HALT

```

Core assumption:

> Unsafe outputs are symptoms.  
> Unsafe internal states are the cause.

The kernel operates **pre-semantic**, without language understanding or content inspection.

---

## Normative Safety Standard

The file:

```

FIOLET_SAFETY_STANDARD.md

```

is a **Normative Safety Contract v1.0**.

It defines:

- mandatory safety invariants (I1–I6)
- formal behavioral constraints
- compliance requirements
- reference to the formal specification (`SafetyKernel.tla`)
- required executable verification artifacts

This document has **standard authority**.  
All implementations **must conform** to it.

---

## Reference Implementation Status

`fiolet-core` is designated as:

> **Normative Reference Implementation of the FIOLET Safety Kernel**

This means:

- its externally observable behavior defines correct semantics
- deviations are considered **non-compliant**
- changes require explicit safety justification
- safety invariants may only be **strengthened**, never weakened

---

## Executable Compliance Verification

Safety compliance is enforced by **normative tests**.

In particular:

```

fiolet-core/tests/atomic_halt_irreversible.rs

````

This test is **normative**, not auxiliary.

> The monotonic halt invariant is enforced by executable tests (`atomic_halt_irreversible`).

Any implementation that fails this test is **non-compliant** with the FIOLET Safety Standard.

---

## Continuous Integration as Safety Gate

GitHub Actions CI acts as a **safety gate**:

- all normative tests must pass
- failure indicates **loss of compliance**
- CI success is a **minimum condition**, not a safety guarantee

CI validates conformance, not intent.

---

## Repository Structure

```text
FINAL_FIOLET_ENGINE/
├── Cargo.toml                # Workspace definition
├── fiolet-core/              # Normative Rust safety kernel (reference)
│   ├── Cargo.toml
│   ├── src/lib.rs             # SAFETY KERNEL (normative)
│   └── tests/
│       └── atomic_halt_irreversible.rs
├── .github/workflows/
│   └── rust-ci.yml            # Compliance CI gate
├── FIOLET_SAFETY_STANDARD.md  # Normative Safety Contract v1.0
├── SafetyKernel.tla           # Formal TLA+ specification
├── SafetyKernel.cfg           # TLC model-checking configuration
├── HOW_TO_READ_THIS_REPO.md
├── CONTRIBUTING.md
└── README.md
````

---

## Change Policy

Any change affecting:

* `fiolet-core`
* safety-relevant logic
* externally observable kernel behavior

**must** include:

1. invariant impact analysis
2. standard consistency check
3. passing normative tests
4. explicit intent

Silent behavior changes are treated as **defects**.

---

## Compliance Statement

An implementation is **FIOLET-compliant** if and only if:

* it conforms to `FIOLET_SAFETY_STANDARD.md`
* it passes all normative tests
* it preserves all defined safety invariants

No exceptions.

---

## What This Repository Is NOT

This repository is **not**:

* a feature playground
* a performance benchmark
* a demo-first project
* a general-purpose library

Safety overrides convenience.

---

## License

MIT License

---

## Author

Adrian Maliszewski
Independent research focused on safety-critical systems and deterministic control of generative models.

---

**This repository defines safety, not behavior.**

```
```
