# FIOLET Core API Documentation

## Overview
FIOLET Core provides deterministic safety monitoring for AI systems through latent space analysis.

## Core Modules

### `SafetyMonitor`
Main safety monitoring component.

#### Constructor
```rust
pub fn new(threshold: f64) -> Self
```
Creates a new monitor with specified KL divergence threshold.

**Parameters:**
- `threshold`: Maximum allowed KL divergence (typically 0.3-0.7)

#### Methods

**`check_divergence(&mut self, kl_value: f64) -> bool`**
Checks if KL divergence exceeds safety threshold.

**Returns:** `true` if threshold exceeded

**`trigger_halt(&mut self)`**
Immediately halts the system.

**`is_active(&self) -> bool`**
Returns current safety status.

---

### `KLDivergence`
Utility for computing Kullback-Leibler divergence.

#### Methods

**`compute(p: &[f64], q: &[f64]) -> f64`**
Computes D_KL(P || Q) between two probability distributions.

**Formula:**
```
D_KL(P || Q) = Σ P(x) * log(P(x) / Q(x))
```

**`exceeds_threshold(p: &[f64], q: &[f64], tau: f64) -> bool`**
Convenience method combining compute and threshold check.

---

## Usage Example
```rust
use fiolet_core::{SafetyMonitor, KLDivergence};

let mut monitor = SafetyMonitor::new(0.5);
let p = vec![0.6, 0.3, 0.1];
let q = vec![0.5, 0.3, 0.2];

let kl = KLDivergence::compute(&p, &q);
if monitor.check_divergence(kl) {
    monitor.trigger_halt();
}
```

## Safety Guarantees

- **Deterministic:** Same inputs always produce same outputs
- **No heap allocation:** Compatible with `no_std` environments
- **Type-safe:** Leverages Rust's ownership system
