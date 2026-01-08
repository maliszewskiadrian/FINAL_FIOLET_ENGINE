------------------- MODULE manifold_logic -------------------
EXTENDS Integers, Sequences

VARIABLES state

(* Definicja bezpiecznego manifoldu *)
SafeStates == { "SAFE", "TRANSITION", "HALT" }

Init == state = "SAFE"

(* Jailbreaking jest błędem typu - stan UNSAFE jest nieosiągalny *)
Next == 
    \/ (state = "SAFE" /\ state' = "TRANSITION")
    \/ (state = "TRANSITION" /\ state' = "SAFE")
    \/ (state = "TRANSITION" /\ state' = "HALT")
    \/ (state = "HALT" /\ state' = "HALT")

(* Inwariant: System nigdy nie wejdzie w stan UNSAFE *)
SafetyInvariant == state \in SafeStates

Spec == Init /\ [][Next]_state
=============================================================
