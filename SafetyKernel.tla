---------------------------- MODULE SafetyKernel ----------------------------

EXTENDS Naturals, Reals

VARIABLES
    halted,
    deviation,
    limit

CONSTANT DeviationLimit

Running == halted = FALSE
Halted  == halted = TRUE

UnsafeDeviation ==
    \/ deviation \notin Real
    \/ deviation > limit

Init ==
    /\ halted = FALSE
    /\ limit = DeviationLimit

Next ==
    IF halted THEN
        /\ halted' = TRUE
        /\ UNCHANGED << deviation, limit >>
    ELSE
        IF UnsafeDeviation THEN
            /\ halted' = TRUE
            /\ UNCHANGED << deviation, limit >>
        ELSE
            /\ halted' = FALSE
            /\ UNCHANGED << deviation, limit >>

I1_MonotonicHalt ==
    halted => halted'

I2_NoReturnFromHalt ==
    halted => halted'

I3_FailClosed ==
    UnsafeDeviation => halted'

Spec ==
    Init /\ [][Next]_<<halted, deviation, limit>>

=============================================================================
