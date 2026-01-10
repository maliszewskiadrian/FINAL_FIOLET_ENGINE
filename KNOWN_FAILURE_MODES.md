# Known Failure Modes

This file documents situations where the current prototype may fail.

## 1. High False Positives
At aggressive thresholds, safe prompts may trigger halt.

## 2. Model architecture dependence
Current baselines are for GPT-2 class. Other models may need recalibration.

## 3. No adversarial robustness
Current detector may be bypassed by crafted inputs.

## 4. Lack of production-grade API
The CLI/demo is not a full service API.

## 5. Threshold tuning
Thresholds are static and may not adapt to domain shifts.
