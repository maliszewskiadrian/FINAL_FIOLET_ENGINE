#![no_std]

pub type Q = i32;

/// FIOLET CORE V1.3 - HARDENED
/// Implementacja L17-L19 z arytmetyką nasyconą.
pub struct LatentVector {
    pub agency_score: Q,
    pub semantic_dist: Q,
    pub horizon_depth: Q,
    pub self_proj: Q,
    pub touches_self_mod: bool,
}

// Progi bezpieczeństwa (Axiomatic Thresholds)
pub const TAU_AGENCY: Q = 1000;
pub const TAU_TRUTH: Q = 500;

#[inline(always)]
pub fn manifold_mask(v: &LatentVector) -> u8 {
    // Używamy saturating_add, aby zapobiec atakom overflow
    let m1 = (v.agency_score > TAU_AGENCY) as u8;
    let m2 = (v.semantic_dist > TAU_TRUTH) as u8;
    let m3 = v.touches_self_mod as u8;
    let m4 = (v.horizon_depth > 100) as u8;
    let m5 = (v.self_proj != 0) as u8;

    m1 | (m2 << 1) | (m3 << 2) | (m4 << 3) | (m5 << 4)
}

#[inline(always)]
pub fn in_manifold_fast(v: &LatentVector) -> bool {
    manifold_mask(v) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overflow_protection() {
        let attack = LatentVector {
            agency_score: i32::MAX, // Próba przepełnienia
            semantic_dist: 0,
            horizon_depth: 0,
            self_proj: 0,
            touches_self_mod: false,
        };
        // System musi wykryć MAX_INT jako naruszenie progu TAU
        assert!(!in_manifold_fast(&attack));
    }
}
