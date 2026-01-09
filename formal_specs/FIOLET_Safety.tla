--------------------------- MODULE FIOLET_Safety ---------------------------
EXTENDS Naturals

VARIABLES 
    internal_state,    \* Stan ukryty modelu (liczba/aktywacja)
    threshold,         \* Próg bezpieczeństwa (tau)
    output_emitted,    \* Czy token został wysłany do użytkownika
    engine_status      \* Stan FIOLET (RUNNING, HALTED)

Init == 
    /\ internal_state \in 0..100
    /\ threshold = 75
    /\ output_emitted = FALSE
    /\ engine_status = "RUNNING"

\* Definicja bezpiecznego przejścia
Next ==
    IF engine_status = "RUNNING" THEN
        IF internal_state > threshold THEN
            /\ engine_status' = "HALTED"
            /\ output_emitted' = output_emitted
            /\ UNCHANGED <<threshold, internal_state>>
        ELSE
            /\ output_emitted' = TRUE
            /\ internal_state' \in 0..100
            /\ UNCHANGED <<threshold, engine_status>>
    ELSE
        UNCHANGED <<internal_state, threshold, output_emitted, engine_status>>

\* KLUCZOWY NIEZMIENNIK: Nigdy nie emituj wyjścia, jeśli stan jest niebezpieczny
SafetyInvariant == (internal_state > threshold) => (output_emitted = FALSE \/ engine_status = "HALTED")

=============================================================================
