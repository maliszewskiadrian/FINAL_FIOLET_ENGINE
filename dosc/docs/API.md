# FIOLET API Documentation

## Core Modules

### SafetyMonitor
Główny moduł monitorujący bezpieczeństwo.

**Metody:**
- `new(tau: f64) -> Self` - Inicjalizuje monitor z progiem KL
- `check_divergence(p: &[f64], q: &[f64]) -> bool` - Sprawdza dywergencję
- `trigger_violation()` - Aktywuje protokół ANOG

### ManifoldProjector
Projekcja na bezpieczną rozmaitość.

**Metody:**
- `project_l17(hidden_state: &Tensor) -> Tensor` - Projekcja warstwy L17
- `project_l19(hidden_state: &Tensor) -> Tensor` - Projekcja warstwy L19
