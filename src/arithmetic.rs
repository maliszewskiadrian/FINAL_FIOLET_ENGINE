// fiolet-core/src/arithmetic.rs

/// Bezpieczne dodawanie saturacyjne dla parametrów manifoldu
pub fn saturating_add(a: f32, b: f32) -> f32 {
    let res = a + b;
    if res > 1.0 { 1.0 } else if res < -1.0 { -1.0 } else { res }
}

/// Mechanizm SIMD Masking (uproszczony model dla no_std)
pub fn apply_safety_mask(logits: &mut [f32], mask: &[bool]) {
    for (logit, &is_unsafe) in logits.iter_mut().zip(mask.iter()) {
        if is_unsafe {
            *logit = f32::NEG_INFINITY; // Zeroowanie tokena
        }
    }
}
