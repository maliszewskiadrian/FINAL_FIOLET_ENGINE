# HOW TO READ THIS REPOSITORY

This document explains the main folders and where to start.

## 📌 Main folders

- `fiolet-python/` — core implementation in Python (activation monitor & decision logic)
- `fiolet-rust/` — experimental Rust performance core
- `experiments/` — scripts for model evaluation and running the detector
- `notebooks/` — exploratory data analysis and visualizations
- `formal_specs/` — formal specs (TLA+ or future formal logic)
- `demos/` — runnable demos

## 🚀 Where to start

1. Browse `fiolet-python/` to understand the core logic.
2. Run the demo: `python demos/basic_demo.py`
3. Explore experiments to see evaluation results.

## 📘 Terminology

- **Activation Monitor** — collects internal states of the model.
- **Deviation Detector** — detects statistical anomalies.
- **ATOMIC HALT** — immediate stop of generation if unsafe detected.

## 📎 Notes

This project focuses on *pre-semantic safety* — looking at model internals, not filtering text after generation.
