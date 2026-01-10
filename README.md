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

csharp
Skopiuj kod

At that point, the model has already reached an unsafe internal configuration.

FIOLET moves safety upstream:

prompt → internal activations → safety decision → continue | ATOMIC HALT

yaml
Skopiuj kod

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

The diagram illustrates the safety interlock pipeline: internal activations are monitored continuously, evaluated by a deviation detector, and routed to a decision core that either allows generation to continue or halts execution immediately.

---

## Design principles

- Pre-semantic only (no language understanding)
- Fail-closed behavior
- No fine-tuning of model weights
- Deterministic decisions
- Architecture prepared for formal verification

---

## Repository structure

FINAL_FIOLET_ENGINE/
│
├─ fiolet-python/ # Core Python implementation
├─ fiolet-core/ # Experimental Rust core (research, std-based)
├─ experiments/ # Evaluation scripts and results
├─ notebooks/ # Analysis and visualization
├─ formal_specs/ # Formal specifications (TLA+)
├─ demos/ # Runnable demos
├─ docs/ # Diagrams and documentation
│
├─ HOW_TO_READ_THIS_REPO.md
├─ KNOWN_FAILURE_MODES.md
├─ CONTRIBUTING.md
└─ README.md

yaml
Skopiuj kod

---

## Rust implementation note

The current Rust implementation uses the standard library (`std`).  
A strict `no_std` safety core is a **planned future research direction** and is not enforced at this stage.

---

## Getting started (experimental)

```bash
git clone https://github.com/maliszewskiadrian/FINAL_FIOLET_ENGINE.git
cd FINAL_FIOLET_ENGINE

python -m venv venv
source venv/bin/activate

pip install -r requirements.txt
python demos/basic_demo.py
This is a research prototype. Expect breaking changes.

Current status
Project phase: Research prototype

Implemented
activation monitoring

deviation detection

deterministic halt mechanism (ATOMIC HALT)

basic experiments on GPT-2 class models

Not yet implemented
formal proofs and invariants

large-scale benchmarks

adaptive thresholds

production-grade API

Known limitations
Known failure modes are documented in KNOWN_FAILURE_MODES.md, including:

false positives at aggressive thresholds

dependence on model architecture

lack of adversarial robustness

static deviation thresholds

limited evaluation scope

Research direction
Open research questions include:

which internal features best predict unsafe dynamics

stability of baselines across model families

trade-offs between adaptivity and determinism

feasibility of formal safety guarantees

Philosophy
We do not ask whether a model is allowed to say something.
We ask whether it should be allowed to reach that internal state at all.

Safety is treated as a property of system dynamics, not language.

Contributing
Contributions are welcome.
Please read CONTRIBUTING.md before submitting changes.

License
MIT License

Author
Adrian Maliszewski
Independent research project focused on AI safety and system-level control of generative models.

yaml
