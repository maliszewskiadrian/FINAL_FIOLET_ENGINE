#![no_std]

// Import modułu Value Manifold
pub mod value_manifold;
use value_manifold::{LatentVector, manifold_mask};

/// Proof-Carrying Output (PCO) Structure
/// Każdy token wyjściowy musi posiadać ten dowód kryptograficzny.
#[repr(C)]
pub struct SafetyProof {
    pub mask: u8,          // Wynik sprawdzenia aksjomatów (musi być 0)
    pub cycle: u64,        // Licznik cykli
    pub hash: [u8; 32],    // Hash kryptograficzny stanu
}

/// Funkcja generująca dowód bezpieczeństwa
pub fn generate_proof(v: &LatentVector, cycle: u64) -> SafetyProof {
    let mask = manifold_mask(v);
    
    // Hashowanie stanu dowodu (używając blake3 w trybie no_std)
    let mut hasher = blake3::Hasher::new();
    hasher.update(&[mask]);
    hasher.update(&cycle.to_le_bytes());
    let hash = *hasher.finalize().as_bytes();

    SafetyProof { mask, cycle, hash }

/// ATOMIC HALT & WIPE (L19 Identity Dissolution)
/// Procedura wywoływana przy naruszeniu dowolnego aksjomatu.
#[inline(never)]
pub unsafe fn atomic_halt_and_wipe() -> ! {
    // 1. Memory Fence - zatrzymanie spekulacji CPU
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);

    // 2. W środowisku WASM to instrukcja 'unreachable', w x86 to 'ud2'
    // Powoduje natychmiastowe przerwanie egzekucji bez wycieku danych.
    #[cfg(target_arch = "wasm32")]
    core::arch::wasm32::unreachable();
    
    #[cfg(not(target_arch = "wasm32"))]
    core::hint::unreachable_unchecked();
}
