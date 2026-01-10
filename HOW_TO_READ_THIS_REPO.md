# How to Read This Repository

This document explains how the FINAL FIOLET ENGINE repository is structured and how to approach it as a reader.

The project is a research prototype.  
Not all components are equally mature.

---

## Recommended reading order

If you are new to this repository:

1. Read `README.md` for the high-level idea
2. Read `KNOWN_FAILURE_MODES.md` to understand limitations
3. Browse `fiolet-python/` for core logic
4. Run `demos/basic_demo.py`
5. Explore `experiments/` and `notebooks/` for evaluation and analysis

---

## Repository structure overview

FINAL_FIOLET_ENGINE/
│
├─ fiolet-python/ Core Python implementation
├─ fiolet-rust/ Experimental high-performance core
├─ experiments/ Evaluation scripts and result summaries
├─ notebooks/ Analysis, plots, and exploration
├─ formal_specs/ Formal specifications (TLA+ and related)
├─ demos/ Minimal runnable demos
├─ docs/ Diagrams and documentation
│
├─ HOW_TO_READ_THIS_REPO.md
├─ KNOWN_FAILURE_MODES.md
├─ CONTRIBUTING.md
└─ README.md


---

## Core directories explained

### `fiolet-python/`

This directory contains the main reference implementation:
- activation monitoring
- deviation detection
- decision logic
- ATOMIC HALT mechanism

This is the best place to understand how the system works end-to-end.

---

### `fiolet-rust/`

Experimental Rust implementation focused on:
- performance
- lower-level control
- future production feasibility

This code is exploratory and may be incomplete.

---

### `experiments/`

Contains:
- evaluation scripts
- baseline measurements
- small experimental results

Results here are illustrative, not definitive.

---

### `notebooks/`

Used for:
- data analysis
- visualization
- exploratory research

Notebooks may be messy by design; they reflect the research process.

---

### `formal_specs/`

Contains:
- formal specifications
- early attempts at defining safety invariants

These specs are incomplete but represent the long-term direction of the project.

---

### `demos/`

Minimal examples showing:
- how to run the system
- how ATOMIC HALT is triggered

Good starting point for hands-on testing.

---

### `docs/`

Documentation assets:
- diagrams
- supporting explanations
- future design documents

---

## What to expect

- not all code paths are finished
- some experiments are partial
- documentation evolves with the project

This repository values **clarity and honesty over polish**.

---

## How to interpret results

Results in this repository:
- are exploratory
- should not be over-generalized
- are meant to guide further research

Claims are intentionally conservative.

---

## Final note

FINAL FIOLET ENGINE is best read as a **research notebook turned into a repository**, not as a finished framework.

If you are reviewing this project, please treat it as an open investigation rather than a finalized solution.

