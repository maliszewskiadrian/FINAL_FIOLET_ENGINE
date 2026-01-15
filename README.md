![Status](https://img.shields.io/badge/status-safety--critical%20kernel-critical)
![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)
![License: MIT](https://img.shields.io/badge/License-MIT-green)

# FINAL FIOLET ENGINE

FINAL FIOLET ENGINE is a **safety-critical systems research project** focused on building a **deterministic, fail-closed safety interlock** for generative AI systems.

At the center of the repository is **`fiolet-core`** — a **minimal Rust safety kernel** whose behavior is formally constrained, test-enforced, and treated as **normative**.

This is **not an application repository**.  
It defines **what safe behavior means**, not how to build features around it.

---

## Project Status

- Fiolet Core and AgeOfDarkness integration is **fully functional**.
- Normative tests pass:  
  - `fiolet_conformance_halt.rs` ✅
- Core safety invariants are enforced.
- CI validates conformance automatically.

---

## Core component: `fiolet-core`

`fiolet-core` is a **deterministic, monotonic, fail-closed safety kernel** written in Rust.

Properties:

- deterministic behavior
- monotonic halt (halt is irreversible)
- explicit safety decisions (`Continue` | `AtomicHalt`)
- no hidden state
- `no_std` compatible
- `panic = abort` semantics in kernel context

---

## Safety decisions

- `Continue`  
- `AtomicHalt`  

No recovery or partial halt is possible.

---

## Normative safety standard

Defined in:

FIOLET_SAFETY_STANDARD.md


This is a **binding safety contract** specifying:  

- mandatory invariants
- kernel behavior
- compliance tests (executed in `fiolet-core/tests/`)
- reference to formal model (`SafetyKernel.tla`)

---

## Repository Structure

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
│       └── fiolet_conformance_halt.rs
├── .github/workflows/rust-ci.yml
├── FIOLET_SAFETY_STANDARD.md
├── SafetyKernel.tla
├── SafetyKernel.cfg
├── HOW_TO_READ_THIS_REPO.md
├── CONTRIBUTING.md
└── README.md

An implementation is FIOLET-compliant if:

It conforms to FIOLET_SAFETY_STANDARD.md.

Passes all normative tests.

Preserves all defined invariants.

## License

MIT License

## Author

Adrian Maliszewski
Independent research on safety-critical systems and deterministic control of generative AI.


