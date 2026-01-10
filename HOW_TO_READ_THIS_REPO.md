# HOW TO READ THIS REPOSITORY

This document explains the main folders and where to start.

## 📌 Main folders

- `fiolet-python/` — core implementation in Python (activation monitor & decision logic)
- `fiolet-rust/` — experimental Rust performance core
- `experiments/` — evaluation scripts & tests
- `notebooks/` — analysis and visualization
- `formal_specs/` — formal specifications (TLA+)
- `demos/` — runnable demos
- `docs/` — diagrams and documentation

## 🚀 Where to start

1. Look at `fiolet-python/` to understand the core logic.
2. Run the demo: `python demos/basic_demo.py`
3. Explore `experiments/` for evaluation results.

## 📘 Terminology

- **Activation Monitor** — collects and normalizes internal states from the model.
- **Deviation Detector** — detects statistical anomalies.
- **Decision Core** — makes safety decisions.
- **ATOMIC HALT** — immediately stops generation.
