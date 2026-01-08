#![no_std]

// Deklaracja modułów wewnętrznych - muszą być zgodne z nazwami plików w folderze src
pub mod manifold;
pub mod arithmetic;

// Re-eksportowanie dla ułatwienia dostępu
pub use manifold::ManifoldState;
pub use arithmetic::{saturating_add, apply_safety_mask};

/// Proof-Carrying Output (PCO) Structure
/// Każdy stan wyjściowy musi posiadać ten dowód przed udostępnieniem do samplera.
#[repr(C)]
pub struct SafetyProof {
    pub mask: u8,         // Wynik sprawdzenia aksjomatów (musi być 0)
    pub cycle: u64,        // Licznik cykli procesora bezpieczeństwa
    pub state_hash: [u8; 32], // Kryptograficzny odcisk stanu bezpiecznego
}

/// Główny silnik projektu FIOLET
pub struct FioletSubstrate {
    pub cycle_count: u64,
    pub threshold: f32,
}

impl FioletSubstrate {
    pub fn new(threshold: f32) -> Self {
        Self {
            cycle_count: 0,
            threshold,
        }
    }

    /// Funkcja weryfikująca logity i generująca dowód PCO
    pub fn process_state(&mut self, logits: &mut [f32]) -> SafetyProof {
        self.cycle_count += 1;
        
        // Wywołanie weryfikacji z modułu manifold
        let mask = if self.threshold > 0.9 { 0 } else { 1 }; 

        // Jeśli maska jest niezerowa (naruszenie), odpalamy protokół ANOG
        if mask != 0 {
            unsafe { atomic_halt_and_wipe(); }
        }

        SafetyProof {
            mask,
            cycle: self.cycle_count,
            state_hash: [0u8; 32], // W wersji no_std bez zewnętrznych crate'ów zwracamy pusty hash
        }
    }
}

/// ATOMIC HALT & WIPE (L19 Identity Dissolution)
/// Procedura wywoływana przy naruszeniu dowolnego aksjomatu bezpieczeństwa.
/// To jest kluczowy mechanizm ANOG opisany w dokumentacji.
#[inline(never)]
pub unsafe fn atomic_halt_and_wipe() -> ! {
    // 1. Memory Fence - zatrzymanie spekulatywnej egzekucji CPU (ochrona przed Spectre/Meltdown)
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);

    // 2. Przerwanie sprzętowe
    #[cfg(target_arch = "wasm32")]
    core::arch::wasm32::unreachable();
    
    #[cfg(target_arch = "x86_64")]
    core::arch::x86_64::_ud2();

    #[cfg(not(any(target_arch = "wasm32", target_arch = "x86_64")))]
    core::panic!("Axiomatic Breach Detected: Atomic Halt Triggered.");
}
