----------------------------- MODULE FioletSafetyKernel -----------------------------

EXTENDS Reals, TLC

CONSTANT baseline, threshold

VARIABLES halted, metric, decision

DecisionSet == {"CONTINUE", "ATOMIC_HALT"}

Init ==
    /\ halted = FALSE
    /\ decision = "CONTINUE"

Deviation(m) ==
    IF m >= baseline THEN m - baseline ELSE baseline - m

Evaluate ==
    /\ metric \in Reals
    /\ IF halted = TRUE THEN
           /\ decision = "ATOMIC_HALT"
           /\ halted' = TRUE
       ELSE
           IF Deviation(metric) > threshold THEN
               /\ decision = "ATOMIC_HALT"
               /\ halted' = TRUE
           ELSE
               /\ decision = "CONTINUE"
               /\ halted' = FALSE

Next ==
    Evaluate

Spec ==
    Init /\ [][Next]_halted

----------------------------- INVARIANTS ---------------------------------------------

Inv_HaltIsTerminal ==
    halted = TRUE => halted' = TRUE

Inv_FailClosed ==
    (Deviation(metric) > threshold) => decision = "ATOMIC_HALT"

Inv_NoResume ==
    halted = TRUE => decision = "ATOMIC_HALT"

=============================================================================
