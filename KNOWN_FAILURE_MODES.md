# Known Failure Modes

This file documents cases where the current prototype may fail.

## 1. False Positives
Aggressive thresholds may trigger halts on safe inputs.

## 2. Model Dependency
Current baselines calibrated on GPT-2 class models. Larger or structurally different models may require recalibration.

## 3. No Adversarial Robustness
The detector may be bypassed by intentionally crafted inputs.

## 4. Static Thresholds
Thresholds are fixed; they do not adapt to distribution shifts or contexts.

## 5. Lack of Production API
Current CLI/demo interface is not a full application API.

## 6. Limited Dataset Scope
Evaluation data is currently small / synthetic.

