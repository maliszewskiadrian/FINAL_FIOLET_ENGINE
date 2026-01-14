# RFC-0003: Extension & Compliance Classification

Status: ACCEPTED  
Standard: FIOLET – AgeOfDarkness  
Applies to: All post–v1.0 work  
Type: Governance / Compliance  
Author: Adrian Maliszewski  
Date: 2026-01-14

---

## 1. PURPOSE

This RFC defines a **formal classification system**
for extensions, integrations, and compliance claims
related to the FIOLET standard.

Its goal is to prevent:
- semantic dilution
- marketing misuse
- partial or deceptive compliance claims

---

## 2. CORE PRINCIPLE

> Compliance is binary.
> Extension is constrained.
> Interpretation is forbidden.

No component may redefine what FIOLET “means”.

---

## 3. EXTENSION CLASSES

### CLASS E0 — NON-SEMANTIC EXTENSION

Includes:
- tests
- documentation
- tooling
- bindings
- adapters

Rules:
- MUST NOT modify ESAL / META-ESAL / ESV / ETT semantics
- MAY be included in minor versions (v1.x)
- Requires RFC, but NON-BREAKING

---

### CLASS E1 — SEMANTIC EXTENSION (CONTROLLED)

Includes:
- new epistemic modules
- additional invariants
- new termination classes

Rules:
- MUST be explicitly defined as SEMANTIC
- Targets v2.0 or higher
- Requires BREAKING RFC
- Requires full re-audit

---

### CLASS E2 — FORBIDDEN EXTENSION

Includes:
- heuristic fallbacks
- probabilistic overrides
- narrative safety layers
- “best effort” modes
- consciousness claims

Rules:
- Explicitly prohibited
- Cannot claim FIOLET relation
- No RFC path available

---

## 4. COMPLIANCE LEVELS

### C0 — NON-COMPLIANT

- No conformance adapter
- No ETT enforcement
- Any output after HALT

Forbidden to reference FIOLET.

---

### C1 — PARTIAL INTEGRATION

- Uses FIOLET concepts
- Does NOT enforce HALT
- Adapter bypassable

May reference FIOLET,  
MUST NOT claim compliance.

---

### C2 — FULL COMPLIANCE

Requirements:
- Implements conformance adapter
- Passes all ETT tests unmodified
- Enforces silence on HALT
- No engine-level overrides

Only C2 systems may claim:
> “FIOLET-compliant”

---

## 5. COMPLIANCE CLAIM RULES

- Compliance claims are normative statements
- False claims are standard violations
- “Inspired by FIOLET” ≠ compliant
- Silence on HALT is mandatory

---

## 6. AUDITABILITY REQUIREMENT

A C2-compliant system MUST allow:
- independent audit
- reproduction of HALT cases
- verification of silence

Refusal to audit ⇒ loss of compliance claim.

---

## 7. FINAL NOTE

FIOLET is not a philosophy.
It is not an alignment layer.
It is not a safety narrative.

It is a **mechanical refusal to guess**.

This RFC exists to protect that refusal.

---

END OF RFC-0003
