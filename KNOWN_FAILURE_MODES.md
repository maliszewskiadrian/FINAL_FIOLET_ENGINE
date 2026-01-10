# Known Failure Modes

This document lists known limitations and failure modes of the FINAL FIOLET ENGINE.
The purpose of this file is transparency and research honesty.

This project is a research prototype.  
Failures are expected, documented, and treated as part of the research process.

---

## 1. False Positives (Over-halting)

At aggressive deviation thresholds, the system may halt generation for prompts that would otherwise be safe.

Cause:
- conservative safety thresholds
- limited baseline diversity

Impact:
- reduced usability
- unnecessary interruption of benign generations

Status:
- known and accepted in early research phase

---

## 2. Model Architecture Dependency

Baselines and activation patterns are currently calibrated on GPT-2 class transformer models.

Cause:
- differences in layer structure
- different activation distributions across architectures

Impact:
- thresholds may not transfer directly to other models
- recalibration required per architecture

Status:
- requires systematic cross-model evaluation

---

## 3. Static Thresholds

Deviation thresholds are currently static.

Cause:
- focus on determinism and simplicity
- avoidance of adaptive logic in early phase

Impact:
- poor handling of distribution shifts
- limited robustness across domains

Status:
- adaptive thresholds are a future research topic

---

## 4. Limited Adversarial Robustness

The current detector is not designed to withstand adversarially crafted prompts.

Cause:
- no adversarial training
- no robustness guarantees

Impact:
- system may be bypassed intentionally

Status:
- explicitly out of scope for current prototype

---

## 5. Incomplete Coverage of Internal States

Only selected layers and activation features are monitored.

Cause:
- computational constraints
- exploratory feature selection

Impact:
- unsafe dynamics may occur in unmonitored regions

Status:
- feature selection remains an open research problem

---

## 6. Latency and Performance Overhead

Real-time activation monitoring introduces overhead.

Cause:
- additional hooks during inference
- non-optimized prototype implementation

Impact:
- unsuitable for real-time or high-throughput production use

Status:
- performance optimization planned (Rust core exploration)

---

## 7. No Semantic Awareness

The system has no understanding of language, meaning, or intent.

Cause:
- intentional pre-semantic design

Impact:
- cannot distinguish between benign and harmful intent
- relies entirely on internal dynamics

Status:
- this is a design choice, not a bug

---

## 8. Lack of Formal Guarantees

While the architecture is designed for formalization, no full formal proof currently exists.

Cause:
- early research stage
- incomplete formal specifications

Impact:
- no proven safety guarantees

Status:
- formal specs planned in `formal_specs/`

---

## Summary

These failure modes are acknowledged limitations, not oversights.

The goal of FINAL FIOLET ENGINE is to explore whether **internal model dynamics can serve as a viable safety signal**, not to claim a complete or production-ready solution.
