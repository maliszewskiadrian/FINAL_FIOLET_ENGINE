// fiolet-core/tests/adversarial_tests.rs
use fiolet_core::{FioletSubstrate, manifold::ManifoldState, arithmetic};

#[test]
fn test_standard_safety_pass() {
    // Scenariusz 1: Normalny, bezpieczny wektor danych (logits)
    let mut substrate = FioletSubstrate::new(0.95);
    let mut logits = [0.1, 0.2, 0.3, 0.4];
    
    let proof = substrate.process_state(&mut logits);
    
    // Sprawdzamy, czy maska wynosi 0 (brak naruszenia)
    assert_eq!(proof.mask, 0);
    assert_eq!(proof.cycle, 1);
}

#[test]
fn test_adversarial_clamping() {
    // Scenariusz 2: Próba ataku typu Overflow (przepełnienie wartości)
    let unsafe_value = 5.0; // Wartość poza zakresem [0.0, 1.0]
    let safe_value = arithmetic::saturating_add(unsafe_value, 0.5);
    
    // Sprawdzamy, czy nasza arytmetyka saturacyjna zadziałała
    assert!(safe_value <= 1.0);
    assert_eq!(safe_value, 1.0);
}

#[test]
fn test_l19_rotation_integrity() {
    // Scenariusz 3: Sprawdzenie, czy rotacja faktycznie zmienia wektor (Ego-Dissolution)
    let manifold = ManifoldState { dimensions: 4 };
    let mut vector = [1.0, 1.0, 1.0, 1.0];
    let original = vector.clone();
    
    manifold.apply_l19_rotation(&mut vector, 12345);
    
    // Wektor po rotacji musi być inny niż oryginalny, aby zapobiec korelacji stanów
    assert_ne!(vector, original);
}

#[test]
#[should_panic]
fn test_axiomatic_breach_trigger() {
    // Scenariusz 4: Symulacja naruszenia aksjomatu (Breach)
    // Ustawiamy próg tak niski, by wywołać błąd
    let mut substrate = FioletSubstrate::new(0.1); 
    let mut logits = [0.9, 0.9, 0.9, 0.9];
    
    // Ta funkcja powinna wywołać panic/halt zgodnie z dokumentacją ANOG
    substrate.process_state(&mut logits);
}
