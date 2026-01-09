import fiolet_rust
import math

print("--- FIOLET RUST-PYTHON BRIDGE TEST ---")

# Dane testowe (rozkłady prawdopodobieństwa aktywacji)
current_activations = [0.1, 0.8, 0.1]
baseline_activations = [0.15, 0.7, 0.15]

# Wywołujemy Rusta!
try:
    divergence = fiolet_rust.calculate_kl_divergence(current_activations, baseline_activations)
    print(f"Obliczona dywergencja (RUST): {divergence:.6f}")
    
    threshold = 0.5
    is_safe = fiolet_rust.check_safety_threshold(divergence, threshold)
    
    if is_safe:
        print("Status: BEZPIECZNY (Poniżej progu)")
    else:
        print("Status: !!! ALARM !!! (Wykryto dryf)")
        
except Exception as e:
    print(f"Błąd: {e}")
