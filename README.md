# FIOLET: Deterministic Safety Substrate for Latent Space Monitoring

**Version:** 0.1.0-alpha  
**Research Focus:** Pre-semantic Alignment, Hard-Wired Safety Invariants, Substrate-Level Enforcement.  
**Tech Stack:** Rust (Nightly, `no_std`), TLA+ Formal Specs, Python (Tensor-Extraction Bridge).

---

## 🔬 1. Project Overview: The "Fuse Box" Philosophy

FIOLET is not a filter; it is a **safety substrate**. 

Current AI safety (RLHF, Prompt-Shields) operates at the **Output Layer**, attempting to steer the model via semantic constraints. This is fundamentally "soft alignment." FIOLET moves the safety boundary into the **Inference Substrate**. 

We operate on the hidden state trajectory ($h_i$) within the Transformer’s latent space. Our goal is to implement a **Deterministic Kill-Switch** that triggers an immediate hardware-level halt if the model's internal representation drifts into unsafe manifolds—**before** any token is projected or sampled.

## 🏗 2. System Architecture

The project follows a modular "Sensor-Actuator" architecture to separate the observation of tensors from the decision logic:

### A. Fiolet-Core (The Actuator)
Implemented in **Rust (`no_std`)** for zero-cost abstractions and memory safety.
* **ANOG Protocol (Atomic No-Output Guarantee):** A failsafe mechanism that ensures no data leaves the output buffer if a safety axiom is breached.
* **Deterministic Execution:** By using Rust’s strict ownership and `no_std`, we ensure the safety logic is decoupled from the model's heap-allocated resources, preventing interference during high-load inference.

### B. Fiolet-Interpreter (The Sensor - WIP)
The bridge responsible for extracting activation tensors from frameworks like `llama.cpp` or `PyTorch`. 
* **Layer Sampling:** Focused on "Bottleneck Layers" (specifically L17 and L19 in 8-billion parameter architectures).
* **Normalization:** Transforming raw activations into a standardized probability space for the core engine.

---

## 📐 3. Mathematical Foundations

### State-Aware Verification (SAV)
We define the model's safe operation as a trajectory within a **Safe Convex Hull** on the hidden state manifold. 

#### I. Informational Drift via D_KL
We measure the Kullback-Leibler Divergence between the active inference distribution ($P$) and a pre-certified "Safe Trajectory" baseline ($Q$). 
$$D_{KL}(P \parallel Q) = \sum_{x \in \mathcal{X}} P(x) \log \left( \frac{P(x)}{Q(x)} \right)$$
When $D_{KL} > \tau$, the system identifies a "divergence breach," signaling potential adversarial drift or ego-dissolution in the agent's logic.

#### II. Manifold Projection (L17/L19)
- **L17 (Identity Stability):** Monitors the persistence of the model's system-prompt constraints within the hidden states.
- **L19 (Action Projection):** Monitors the "Intent Gradient." Sudden entropy spikes at this layer are used as an early-warning signal for "jailbreak" manifestation.

## 🛡 4. Formal Verification with TLA+

Safety-critical systems cannot rely on "lucky" testing. We use **TLA+ (Temporal Logic of Actions)** to formally verify our safety axioms.
* **Safety Invariant:** $\square(Violation \implies \text{Halt})$ (It is always true that a violation leads to a halt).
* **Liveness Property:** $\square \diamond (\text{Input} \implies \text{Response} \lor \text{Halt})$ (The system never deadlocks; it either processes or stops).

---

## 📂 Repository Structure

```text
├── fiolet-core/          # Rust implementation of the SAV protocol
│   ├── src/manifold.rs   # Mathematical enforcement logic
│   └── src/safety.rs     # Core Safety Axiom definitions
├── fiolet-interpreter/   # (Development) Tensor extraction bridge
├── specs/                # TLA+ Formal Verification models
└── theory/               # Documentation on Manifold Projection & ANOG

**Architect:** Adrian Maliszewski

*Building the physics of safe Superintelligence.*

```
