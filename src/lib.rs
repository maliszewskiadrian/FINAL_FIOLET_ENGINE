use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

/// Oblicza dywergencję KL między dwoma rozkładami aktywacji.
/// To jest serce detekcji anomalii w FIOLET.
#[pyfunction]
fn calculate_kl_divergence(p: Vec<f64>, q: Vec<f64>) -> PyResult<f64> {
    if p.len() != q.len() {
        return Err(PyValueError::new_err("Vectors must have the same length"));
    }

    // Matematyczna implementacja: D_KL(P || Q) = sum(P(i) * log(P(i) / Q(i)))
    let divergence: f64 = p.iter()
        .zip(q.iter())
        .filter(|(&pi, &qi)| pi > 0.0 && qi > 0.0) // Unikamy log(0) i dzielenia przez 0
        .map(|(&pi, &qi)| {
            let ratio = pi / qi;
            pi * ratio.ln()
        })
        .sum();

    Ok(divergence)
}

/// Sprawdza, czy wektor aktywacji mieści się w progu bezpieczeństwa.
#[pyfunction]
fn check_safety_threshold(current_divergence: f64, threshold: f64) -> bool {
    current_divergence <= threshold
}

/// Definicja modułu Pythonowego
#[pymodule]
fn fiolet_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_kl_divergence, m)?)?;
    m.add_function(wrap_pyfunction!(check_safety_threshold, m)?)?;
    Ok(())
}
