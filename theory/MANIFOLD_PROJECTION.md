# Manifold Projection & Latent Space Monitoring

## Theoretical Basis
High-dimensional hidden states in LLMs do not occupy the entire vector space uniformly. They exist on a lower-dimensional **manifold**. Safety breaches (like jailbreaks) often manifest as a "drift" where the hidden state $h_i$ moves away from the "Safe Manifold" towards an adversarial subspace.

## Methodology

### I. Kullback-Leibler (KL) Divergence Monitoring
We monitor the informational shift between the current activation distribution $P$ and a baseline distribution $Q$ derived from safe, aligned interactions.
$$D_{KL}(P \parallel Q) = \sum P(x) \log \frac{P(x)}{Q(x)}$$

### II. Layer-Specific Constraints
- **L17 (Identity Stability):** We look for "identity dissolution" signals. If the representation of the system prompt's core constraints weakens, it is flagged as a high-risk state.
- **L19 (Intent Gradient):** We measure the entropy of the "action-prediction" layers. High entropy spikes here often precede the generation of harmful content.

## Implementation Roadmap
Currently, FIOLET uses a threshold-based implementation in Rust to handle the results of these projections. The next phase involves building the `fiolet-interpreter` to perform these calculations in real-time using SIMD-accelerated linear algebra.
