# FINAL FIOLET ENGINE

![Status](https://img.shields.io/badge/status-research%20prototype-blue)

A pre-semantic safety interlock for generative language models.

---

## Overview

FINAL FIOLET ENGINE is an experimental AI safety research project focused on detecting and interrupting unsafe behavior in language models **before text is generated**.

Instead of analyzing generated content, the system monitors **internal neural activations** during inference and halts generation when the model enters a statistically abnormal or unsafe internal state.

This project treats AI safety as a **systems engineering and control problem**, not a content moderation task.

---

## Motivation

Most existing safety mechanisms operate after generation:

model → generated text → analysis → block

At that point, the model has already reached an unsafe internal configuration.

FIOLET moves safety upstream:

prompt → internal activations → safety decision → continue | ATOMIC HALT

### Core assumption

Unsafe outputs are symptoms.  
Unsafe internal states are the cause.

---

## Core concept

During token generation, FIOLET performs:

1. **Activation monitoring**  
   Selected internal layers of the model are observed in real time.

2. **Deviation detection**  
   Activations are compared against a baseline representing safe operating behavior.

3. **Decision logic**  
   If deviation exceeds a defined threshold, the system triggers **ATOMIC HALT** — an immediate and irreversible stop of generation.

The system is:
- pre-semantic
- non-invasive (no weight modification)
- deterministic
- fail-closed
- designed for formal analysis

---

## High-level architecture

![FIOLET Engine architecture](docs/architecture_overview.png)

Internal activations are continuously monitored, evaluated by a deviation detector, and routed to a decision core that either allows generation to continue or halts execution immediately.

---

## Design principles

- Pre-semantic only (no language understanding)
- Fail-closed behavior
- No fine-tuning of model weights
- Deterministic decisions
- Architecture prepared for formal verification

---

## Repository structure

```text
FINAL_FIOLET_ENGINE/
│
├─ fiolet-core/          # no_std Rust safety kernel (research, fail-closed)
│
├─ fiolet-python/        # Python-side research / integration (experimental)
├─ experiments/          # Evaluation scripts and deviation experiments
├─ notebooks/            # Analysis and visualization
├─ demos/                # Runnable demos and showcases
├─ docs/                 # Diagrams and technical documentation
│   └─ Safety_Kernel_as_a_Standard.md  # Normative safety standard
│
├─ SafetyKernel.tla      # FORMAL SPEC (TLA+): monotonic halt safety kernel
├─ fiolet_core.h         # C ABI contract for the safety kernel (normative)
│
├─ HOW_TO_READ_THIS_REPO.md
├─ KNOWN_FAILURE_MODES.md
├─ CONTRIBUTING.md
└─ README.md
```
## Rust implementation note
The current Rust implementation uses the standard library (std).
A strict no_std safety core is a planned future research direction and is not enforced at this stage.

Rust code is considered experimental and non-authoritative relative to the research concepts.

## Getting started (experimental)

git clone https://github.com/maliszewskiadrian/FINAL_FIOLET_ENGINE.git
cd FINAL_FIOLET_ENGINE

python -m venv venv
source venv/bin/activate

pip install -r requirements.txt
python demos/basic_demo.py

This is a research prototype. Expect breaking changes.

## Current status
Project phase: Research prototype

Implemented
Activation monitoring

Deviation detection

Deterministic halt mechanism (ATOMIC HALT)

Basic experiments on GPT-2 class models

Not yet implemented
Formal proofs and invariants

Large-scale benchmarks

Adaptive thresholds

Production-grade API

## Known limitations
Known failure modes are documented in KNOWN_FAILURE_MODES.md, including:

False positives at aggressive thresholds

Dependence on model architecture

Lack of adversarial robustness

Static deviation thresholds

Limited evaluation scope

## Research direction
Open research questions include:

Which internal features best predict unsafe dynamics

Stability of baselines across model families

Trade-offs between adaptivity and determinism

Feasibility of formal safety guarantees

## Philosophy
We do not ask whether a model is allowed to say something.
We ask whether it should be allowed to reach that internal state at all.

Safety is treated as a property of system dynamics, not language.

## Contributing
Contributions are welcome.
Please read CONTRIBUTING.md before submitting changes.

## License
MIT License

## Author
Adrian Maliszewski
Independent research project focused on AI safety and system-level control of generative models.
