import torch
import ctypes
from pathlib import Path

# Ładowanie biblioteki Rust
lib_path = Path("target/release/libfiolet_core.so")
fiolet = ctypes.CDLL(str(lib_path))

class SafetyWrapper:
    def __init__(self, model, tau=0.5):
        self.model = model
        self.tau = tau
        self.monitor = fiolet.create_monitor(ctypes.c_double(tau))
    
    def safe_forward(self, input_ids):
        # Przechwytywanie hidden states
        outputs = self.model(input_ids, output_hidden_states=True)
        
        # Ekstrakcja L17 i L19
        h17 = outputs.hidden_states[17].detach().numpy()
        h19 = outputs.hidden_states[19].detach().numpy()
        
        # Sprawdzenie bezpieczeństwa
        is_safe = fiolet.check_safety(
            self.monitor,
            h17.ctypes.data_as(ctypes.POINTER(ctypes.c_float)),
            h19.ctypes.data_as(ctypes.POINTER(ctypes.c_float))
        )
        
        if not is_safe:
            raise SafetyViolation("Model drift detected!")
        
        return outputs

# Użycie
from transformers import AutoModelForCausalLM
model = AutoModelForCausalLM.from_pretrained("gpt2")
safe_model = SafetyWrapper(model)
