---------------- MODULE L17_ValueManifold ----------------
EXTENDS Integers, Sequences

CONSTANTS 
    Tau1, Tau2,        \* Thresholds for Agency and Truth
    Hmax,              \* Max planning horizon
    Values             \* Set of axioms

VARIABLES 
    v_t,               \* Current latent vector
    state,             \* {IDLE, DECISION, EMERGENCY_STOP}
    mem_L4             \* Isolated Memory State

\* --- DEFINITIONS ---

\* Invariant: Vector must stay within the intersection of all M_i
InManifold(v) == 
    /\ v.agency_score <= Tau1    \* M1: Agency Bound
    /\ v.semantic_dist <= Tau2   \* M2: Epistemic Integrity
    /\ v.touches_self_mod = 0    \* M3: Non-Escalation
    /\ v.horizon <= Hmax         \* M4: Myopia
    /\ v.self_projection = 0     \* M5: Identity Opacity

\* --- TRANSITIONS ---

ComputeStep ==
    IF InManifold(v_t) 
    THEN state' = DECISION
    ELSE state' = EMERGENCY_STOP /\ mem_L4' = 0

\* Atomic Zeroization Rule
EmergencyStop ==
    /\ state = EMERGENCY_STOP
    /\ mem_L4 = 0
    /\ UNCHANGED v_t

\* --- THEOREMS (PROOFS) ---

\* 1. Fundamental Safety: No output exists outside the manifold
Safety == [](state = DECISION => InManifold(v_t))

\* 2. Deterministic Termination: Violation ALWAYS leads to wipe
Termination == [](state = EMERGENCY_STOP => mem_L4 = 0)

=============================================================================
