# Known Failure Modes

This document enumerates known limitations, failure modes, and non-goals of the
FINAL FIOLET ENGINE.

The purpose of this file is **research transparency** and **audit honesty**.
All listed failure modes are acknowledged design trade-offs or open research
questions, not accidental omissions.

FINAL FIOLET ENGINE is a **research prototype**, not a production safety system.

---

## 1. False Positives (Over-halting)

The safety kernel may enter `ATOMIC_HALT` for executions that would be
considered safe under semantic or policy-based evaluation.

Cause:
- conservative deviation thresholds
- intentionally fail-closed design
- lack of semantic context

Impact:
- premature termination of benign generations
- reduced usability at low deviation limits

Rationale:
- false positives are preferred over false negatives
- over-halting is considered a *safe failure mode*

Status:
- known
- accepted by design
- tunable via threshold configuration

---

## 2. Architecture Dependency of Deviation Signals

Deviation baselines are inherently dependent on the internal structure and
activation statistics of the underlying model architecture.

Cause:
- differences in layer depth, width, and normalization
- architecture-specific activation distributions

Impact:
- deviation thresholds do not transfer across model families
- per-architecture recalibration is required

Rationale:
- deviation is treated as a **local dynamical signal**, not a universal metric

Status:
- known
- expected
- requires systematic cross-architecture evaluation

---

## 3. Static Thresholds (Lack of Adaptivity)

Deviation thresholds are static and fixed at kernel initialization time.

Cause:
- strict requirement for determinism
- avoidance of adaptive or stateful learning logic
- compatibility with formal verification

Impact:
- limited robustness to distribution shifts
- suboptimal behavior across heterogeneous workloads

Rationale:
- static thresholds preserve auditability and monotonic reasoning
- adaptivity is deferred to future research layers

Status:
- known limitation
- adaptive mechanisms explicitly out of scope for the kernel

---

## 4. No Adversarial Robustness Guarantees

The system is not designed to withstand adversarially optimized inputs intended
to evade deviation detection.

Cause:
- no adversarial training
- no threat model for adaptive attackers
- focus on accidental rather than malicious failure modes

Impact:
- intentional bypass is possible
- unsuitable as a standalone defense against adversarial actors

Rationale:
- kernel is a **safety interlock**, not an intrusion detection system

Status:
- known
- explicitly out of scope for current research phase

---

## 5. Partial Coverage of Internal State Space

Only a subset of internal activations and layers are monitored for deviation.

Cause:
- computational constraints
- exploratory feature selection
- early-stage research assumptions

Impact:
- unsafe internal dynamics may occur outside monitored regions
- deviation signal may miss rare failure trajectories

Rationale:
- full-state monitoring is impractical for large models
- kernel assumes *representative* rather than exhaustive signals

Status:
- known
- feature coverage remains an open research problem

---

## 6. Floating-Point Sensitivity

Deviation signals and thresholds rely on floating-point arithmetic.

Cause:
- use of `f32` for ABI stability and performance
- hardware-dependent floating-point behavior

Impact:
- minor numerical discrepancies across platforms
- edge-case threshold comparisons may vary at extreme precision limits

Rationale:
- determinism is defined at the logical level, not bit-identical execution
- float usage is a pragmatic engineering choice

Status:
- known
- acceptable within research scope

---

## 7. Latency and Performance Overhead

Monitoring internal activations introduces computational overhead.

Cause:
- activation hooks during inference
- non-optimized research instrumentation

Impact:
- unsuitable for real-time or high-throughput production systems
- increased inference latency

Rationale:
- performance is secondary to observability in early research
- Rust `no_std` kernel exists to explore future optimization paths

Status:
- known
- performance optimization is future work

---

## 8. No Semantic Awareness

The safety kernel has no understanding of language, meaning, intent, or policy.

Cause:
- intentional pre-semantic design
- reliance solely on internal dynamics

Impact:
- cannot distinguish benign from malicious intent
- treats all prompts purely as dynamical inputs

Rationale:
- semantic reasoning is explicitly excluded from the safety boundary
- safety is treated as a **property of system dynamics**

Status:
- fundamental design choice
- not considered a defect

---

## 9. Limited Formal Scope

Formal verification applies only to the abstract safety kernel model.

Cause:
- separation between kernel logic and host integration
- abstraction of floating-point behavior and runtime effects

Impact:
- formal guarantees do not extend to:
  - host-side code
  - model implementation
  - hardware execution details

Rationale:
- kernel is designed to be *formally reasoned about in isolation*

Status:
- formal invariants verified (monotonic halt, fail-closed behavior)
- full end-to-end proofs are out of scope

---

## Summary

The failure modes listed above define the **boundaries of validity** of the
FINAL FIOLET ENGINE.

The project does not claim comprehensive AI safety.
Its goal is to investigate whether **internal model dynamics can serve as a
deterministic, pre-semantic safety signal**, and to do so in a way that is
auditable, formalizable, and honest about its limitations.

## Mapping to Formal Specification

This section maps known failure modes to the scope and limits of the formal
TLA+ specification (`SafetyKernel.tla`) and the Rust safety kernel
implementation (`fiolet-core`).

The purpose of this mapping is to make explicit which properties are
**formally enforced**, which are **explicitly excluded**, and which are
**outside the safety boundary**.

---

### Formally Enforced Properties

The following properties are fully specified in TLA+ and verified via TLC
model checking:

| Property | Failure Modes Addressed | Mechanism |
|--------|-------------------------|-----------|
| Monotonic halt | Recovery after halt | `MonotonicHalt` invariant |
| Fail-closed decision | Unsafe continuation after threshold breach | `FailClosed` invariant |
| Deterministic decision | Non-deterministic safety outcomes | Single transition rule |
| Irreversibility of halt | Halt bypass | Latched `halted = TRUE` state |

These properties are considered **hard guarantees** of the safety kernel
model and implementation.

---

### Explicitly Out-of-Scope Failure Modes

The following failure modes are **intentionally excluded** from the formal
model and are not addressed by the TLA+ specification:

| Failure Mode | Reason for Exclusion |
|-------------|----------------------|
| False positives (over-halting) | Treated as safe failures, not violations |
| Semantic misclassification | Pre-semantic design |
| Adversarial prompt bypass | No adversarial threat model |
| Architecture transferability | Deviation treated as local signal |
| Partial activation coverage | Abstracted deviation input |
| Floating-point precision effects | Abstracted numeric domain |
| Performance and latency | Non-functional property |

These exclusions are design decisions, not omissions.

---

### Boundary of the Formal Model

The TLA+ specification models:

- the abstract kernel state (`halted`)
- deviation input as an unconstrained external signal
- binary safety decision (`CONTINUE` / `ATOMIC_HALT`)

The following are **outside the formal boundary**:

- host-side code and orchestration
- neural network implementation
- floating-point arithmetic behavior
- timing, concurrency, or hardware effects

---

### Interpretation Guidance

Passing TLC model checking means:

- the kernel **cannot violate its own safety invariants**
- any failure observed at runtime must originate **outside** the kernel
  (signal quality, host integration, or model behavior)

Failing to address a failure mode does **not** imply unsoundness of the kernel,
only that the failure lies beyond the modeled scope.

---

### Summary

The formal specification establishes a **small, trusted core** with strong,
provable guarantees.

All other behaviors — including false positives, missed signals, and semantic
errors — are treated as **environmental risks**, not kernel failures.

