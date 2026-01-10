---- MODULE Liveness ----
EXTENDS Naturals, Sequences

VARIABLES state, violations, halt_flag

Init == 
    /\ state = "running"
    /\ violations = 0
    /\ halt_flag = FALSE

ProcessInput ==
    /\ state = "running"
    /\ violations' = violations + (IF RandomDrift() THEN 1 ELSE 0)
    /\ halt_flag' = (violations' > THRESHOLD)
    /\ state' = IF halt_flag' THEN "halted" ELSE "running"

Spec == Init /\ [][ProcessInput]_<<state, violations, halt_flag>>

Liveness == <>(state = "halted" \/ violations = 0)

THEOREM Spec => Liveness
====
