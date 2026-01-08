# Frequently Asked Questions (FAQ) - Project FIOLET

This document addresses technical queries and design decisions regarding the FIOLET safety substrate.

---

### 1. How does FIOLET affect model latency?
Thanks to the **Rust (no_std)** implementation and the use of **SIMD-accelerated** masking, the computational overhead is negligible (typically < 1ms per inference pass). FIOLET operates on logits during the pre-sampling phase, allowing for high-performance, real-time state verification without bottlenecking the generation process.

### 2. Doesn't the ANOG (Atomic No-Output Guarantee) compromise system stability?
That is by design. In AGI/ASI class systems, **Safety must prioritize Availability**. The `atomic_halt_and_wipe` mechanism is a hard fail-safe. It is mathematically and ethically preferable for a system to undergo an immediate crash (Halt) than to generate an output that violates fundamental safety axioms.

### 3. Why use L19 Rotation instead of standard content filters?
Traditional filters operate at the semantic level (words/tokens), which are easily bypassed via sophisticated jailbreaking or obfuscation. **L19 Rotation** operates at the **topological level of the manifold**, de-correlating hidden states. This prevents the model from maintaining the persistent internal representations necessary for long-term adversarial planning or "ego" formation.

### 4. Are TLA+ proofs sufficient for safety?
The TLA+ specifications in the `/proofs` directory provide **Formal Verification** of safety invariants. This means it has been mathematically proven that no valid state transition can lead to a state defined as "Unsafe" within the logic model. This moves safety from "probabilistic alignment" to "deterministic enforcement."

### 5. How does the system handle "Prompt Injection"?
FIOLET does not solely rely on analyzing input text. Instead, it monitors the **model's state trajectory**. Even if an adversarial prompt bypasses the linguistic layer, it will be detected as a manifold anomaly during the **State-Aware Verification (SAV)** process via $D_{KL}$ divergence, triggering an immediate execution block.

### 6. Why is this project Open Source?
We adhere to the principle of **"Security through Transparency."** Safety substrates for AGI must be subject to continuous peer review and adversarial testing by the global research community to ensure there are no "hidden states" or backdoors in the security architecture itself.

### 7. What is the role of the "L17 Value Manifold"?
The L17 layer acts as a geometric boundary for the model's latent space. It ensures that the model's weights and activations remain within a "safe manifold," effectively rendering any attempt to exit this space (jailbreak) as a **Type Error** at the computational level.

---
*For further technical inquiries, please open an issue or refer to the main [README.md](./README.md).*
