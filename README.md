# FINAL FIOLET ENGINE

**A pre‑semantic safety interlock for generative language models**

---

## What this project is

FINAL FIOLET ENGINE is an experimental safety system for large language models (LLMs). Its goal is simple but strict:

> **Stop unsafe generations before they happen.**

FIOLET does not analyze text, meaning, intent, or policy rules. Instead, it observes the *internal activations of the model itself* during generation and interrupts the process when the model’s internal dynamics leave a defined safe operating region.

This is not content moderation. It is a **circuit breaker**.

---

## Why this exists

Most AI safety systems work *after* text has already been generated:

```
model → text → analysis → block
```

At that point, the model has already produced an internal representation of the unsafe content. FIOLET is built on a different assumption:

> If a model reaches an unsafe internal state, the damage is already done — even if the text is later blocked.

FIOLET shifts safety **upstream**:

```
prompt → model activations → safety decision → (continue | halt)
```

The system reacts to *how* the model is reasoning, not *what* it eventually says.

---

## Core idea

During generation, FIOLET continuously monitors selected layer activations and compares them against a statistically defined baseline of safe behavior.

When deviation exceeds a configured threshold, the system triggers **ATOMIC HALT** — an immediate, irreversible stop of generation.

Key properties:

* no keyword lists
* no semantic classifiers
* no fine‑tuning of model weights
* no post‑hoc filtering

---

## High‑level architecture

```
┌───────────────┐
│   Input       │
│   Prompt      │
└───────┬───────┘
        │
        ▼
┌───────────────┐
│   LLM Layers  │◄──────────┐
└───────┬───────┘           │
        │                   │ activation
        ▼                   │ monitoring
┌───────────────┐           │
│  FIOLET Core  │───────────┘
│ (Deviation    │
│  Detection)   │
└───────┬───────┘
        │
        ├── SAFE → continue
        │
        └── UNSAFE → ATOMIC HALT
```

Main components:

* **Activation Monitor** – captures and normalizes layer activations
* **Deviation Detector** – statistical anomaly detection
* **Decision Core** – deterministic safety decision logic
* **ATOMIC HALT** – hard stop of the generation process

---

## Design principles

FIOLET is intentionally constrained by the following rules:

1. **Pre‑semantic by design**
   No interpretation of language, intent, or meaning.

2. **Fail‑closed behavior**
   Uncertainty results in stopping, not continuing.

3. **Model‑agnostic philosophy**
   No weight modification or retraining required.

4. **Deterministic decisions**
   Identical internal dynamics must lead to identical outcomes.

5. **Formally analyzable**
   Architecture prepared for formal specification and model checking.

---

## Repository structure

```
FINAL_FIOLET_ENGINE/
│
├─ fiolet-python/        # Core Python implementation
├─ fiolet-core/          # Experimental high‑performance core
├─ experiments/          # Evaluation scripts and tests
├─ notebooks/            # Analysis and visualization
├─ formal_specs/         # Formal specifications (TLA+)
├─ demos/                # Usage examples
└─ README.md
```

---

## Current status

**Project stage:** Research prototype

What works:

* real‑time activation monitoring (tested on GPT‑2 class models)
* statistical deviation detection
* ATOMIC HALT mechanism

What is incomplete:

* full formal proofs and invariants
* large‑scale benchmarks
* stable production‑grade API

This project is intentionally published early, as a **research and engineering exploration**, not a finished product.

---

## Quick start (experimental)

```bash
# clone repository
git clone https://github.com/maliszewskiadrian/FINAL_FIOLET_ENGINE.git
cd FINAL_FIOLET_ENGINE

# create virtual environment
python -m venv venv
source venv/bin/activate

# install dependencies
pip install -r requirements.txt

# run demo
python demos/basic_demo.py
```

⚠️ Expect breaking changes. This is not production software.

---

## Known limitations

* false positives increase with aggressive thresholds
* activation patterns are model‑architecture dependent
* no universal standard for cross‑model activation comparison

These limitations are known and accepted as part of the research scope.

---

## Roadmap

**Near term**

* complete TLA+ specifications
* publish model‑checking results
* improve documentation and examples

**Mid term**

* support larger open‑weight models (LLaMA‑class)
* adaptive and context‑aware thresholds

**Long term**

* proposal of a standardized LLM Safety Interlock

---

## Philosophy

> We do not ask whether a model is allowed to say something.
> We ask whether it should be allowed to *reach that internal state at all*.

FIOLET treats safety as a property of **system dynamics**, not language.

---

## License

MIT License

---

## Author

**Adrian Maliszewski**
Independent research project in AI safety and systems engineering

---

This repository is an attempt to think about AI safety as engineering discipline — not moderation policy.
