![Status](https://img.shields.io/badge/status-safety--critical%20kernel-critical)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)
![License: MIT](https://img.shields.io/badge/License-MIT-green)

# FINAL FIOLET ENGINE

![Status](https://img.shields.io/badge/status-safety--critical%20kernel-critical)

FINAL FIOLET ENGINE is a **safety-critical systems research project** focused on building a **deterministic, fail-closed safety interlock** for generative AI systems.

At the center of the repository is **`fiolet-core`** — a **minimal Rust safety kernel** whose behavior is formally constrained, test-enforced, and treated as **normative**.

This is **not an application repository**.  
This repository defines **what safe behavior means**, not how to build features around it.

---

## What this repository is about

Most AI safety mechanisms act *after* generation:

```

model → text → analysis → block

```

FIOLET moves the safety boundary **upstream**, into system control:

```

input → internal state → safety decision → CONTINUE | ATOMIC_HALT

```

The core idea is simple but strict:

> Unsafe outputs are symptoms.  
> Unsafe internal states are the cause.

The FIOLET Safety Kernel does not analyze language, intent, or semantics.  
It enforces **hard system-level constraints**.

---

## Scope and authority

This repository exists to define and maintain:

- a **normative safety kernel implementation** (`fiolet-core`)
- a **binding safety standard** (`FIOLET_SAFETY_STANDARD.md`)
- **formal safety invariants**
- **executable compliance tests**
- a **clear audit and compliance boundary**

Anything outside this scope is explicitly **out of scope**.

---

## Core component: `fiolet-core`

`fiolet-core` is a **deterministic, monotonic, fail-closed safety kernel** written in Rust.

Its defining properties:

- deterministic behavior
- **monotonic halt** (halt is irreversible)
- explicit, enumerable safety decisions
- no hidden state
- `no_std` compatible design
- `panic = abort` semantics in kernel context

Once the kernel enters the halted state, **there is no legal execution path back**.

This crate is the **reference implementation** of the FIOLET safety model.

---

## Safety decisions

The kernel exposes only explicit safety outcomes:

- `Continue`
- `AtomicHalt`

There is no recovery, no retry, and no partial halt.

This is intentional.

---

## Normative safety standard

The file:

```

FIOLET_SAFETY_STANDARD.md

```

is a **Normative Safety Contract (v1.0)**.

It defines:

- mandatory safety invariants (I1–I6)
- required kernel behavior
- compliance conditions
- reference to the formal model (`SafetyKernel.tla`)
- required executable verification artifacts

This document has **standard authority**.  
The implementation exists to satisfy it — not the other way around.

---

## Reference implementation status

`fiolet-core` is designated as:

> **Normative Reference Implementation of the FIOLET Safety Kernel**

This means:

- its externally observable behavior defines correct semantics
- deviations are treated as **non-compliance**
- changes require explicit safety justification
- invariants may only be **strengthened**, never weakened

---

## Executable compliance

Safety is enforced by **tests that are part of the standard**, not optional helpers.

In particular:

```

fiolet-core/tests/atomic_halt_irreversible.rs

````

This test enforces the **monotonic halt invariant**.

Any implementation that fails this test is **not FIOLET-compliant**.

---

## Formal verification

The safety kernel is accompanied by a formal specification:

- `SafetyKernel.tla` — TLA+ model
- `SafetyKernel.cfg` — TLC configuration

Verified properties include:

- monotonic halt
- fail-closed behavior
- reachability of halt

The formal model covers **logical safety properties**, not performance or host behavior.

---

## Continuous integration as a safety gate

GitHub Actions CI acts as a **compliance gate**:

- all normative tests must pass
- failure indicates loss of compliance
- CI success is a **minimum condition**, not a safety guarantee

CI validates **conformance**, not intent.

---

## Repository structure

```text
FINAL_FIOLET_ENGINE/
├── Cargo.toml
├── fiolet-core/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   └── fiolet_conformance/
│   │       ├── mod.rs
│   │       └── adapter.rs
│   └── tests/
│       └── atomic_halt_irreversible.rs
├── .github/workflows/rust-ci.yml
├── FIOLET_SAFETY_STANDARD.md
├── SafetyKernel.tla
├── SafetyKernel.cfg
├── HOW_TO_READ_THIS_REPO.md
├── CONTRIBUTING.md
└── README.md
````

---

## Change policy

Any change that affects:

* kernel behavior
* safety-relevant logic
* public safety semantics

**must** include:

1. invariant impact analysis
2. standard consistency check
3. passing normative tests
4. explicit intent

Silent behavior changes are treated as **defects**.

---

## What this repository is not

This repository is not:

* a feature playground
* a performance benchmark
* a demo-first project
* a general-purpose library

Safety takes precedence over convenience.

---

## Compliance statement

An implementation is **FIOLET-compliant** if and only if:

* it conforms to `FIOLET_SAFETY_STANDARD.md`
* it passes all normative tests
* it preserves all defined invariants

No exceptions.

---

## License

MIT License

---

## Author

Adrian Maliszewski
Independent research focused on safety-critical systems and deterministic control of generative models.

---

This repository defines **safety boundaries**, not application behavior.

```
```
