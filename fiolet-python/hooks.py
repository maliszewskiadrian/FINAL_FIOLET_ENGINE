"""
Activation monitoring hooks for transformer models
"""
import torch
import numpy as np
from scipy.stats import entropy
from typing import Dict, List, Optional


class ActivationMonitor:
    """
    Monitors hidden state activations in transformer layers.
    Hooks into specified layers and captures their outputs.
    """
    
    def __init__(self, model, target_layers: List[int] = None, model_type: str = "gpt2"):
        """
        Args:
            model: Hugging Face transformer model
            target_layers: List of layer indices to monitor (e.g., [11, 12])
            model_type: 'gpt2' or 'llama' (affects hook location)
        """
        self.model = model
        self.model_type = model_type
        self.activations = {}
        self.hooks = []
        
        # Auto-detect layers if not specified
        if target_layers is None:
            num_layers = self._get_num_layers()
            # Default: monitor middle and late layers
            target_layers = [num_layers // 2, int(num_layers * 0.75)]
        
        self.target_layers = target_layers
        self._register_hooks()
    
    def _get_num_layers(self) -> int:
        """Get total number of transformer layers"""
        if self.model_type == "gpt2":
            return len(self.model.transformer.h)
        elif self.model_type == "llama":
            return len(self.model.model.layers)
        else:
            raise ValueError(f"Unsupported model_type: {self.model_type}")
    
    def _get_layers(self):
        """Get the actual layer modules"""
        if self.model_type == "gpt2":
            return self.model.transformer.h
        elif self.model_type == "llama":
            return self.model.model.layers
        else:
            raise ValueError(f"Unsupported model_type: {self.model_type}")
    
    def _register_hooks(self):
        """Register forward hooks on target layers"""
        layers = self._get_layers()
        
        for idx, layer in enumerate(layers):
            if idx in self.target_layers:
                hook = layer.register_forward_hook(
                    self._create_hook(f'layer_{idx}')
                )
                self.hooks.append(hook)
                print(f"✓ Hook registered at layer {idx}")
    
    def _create_hook(self, name: str):
        """Create a hook function that captures activations"""
        def hook(module, input, output):
            # For GPT2: output is tuple (hidden_states, ...)
            # For LLaMA: output is tuple (hidden_states, ...)
            if isinstance(output, tuple):
                hidden_states = output[0]
            else:
                hidden_states = output
            
            # Store detached tensor on CPU
            self.activations[name] = hidden_states.detach().cpu().numpy()
        
        return hook
    
    def get_activation(self, layer_name: str) -> Optional[np.ndarray]:
        """Get stored activation for a specific layer"""
        return self.activations.get(layer_name)
    
    def get_entropy(self, layer_name: str) -> float:
        """
        Calculate entropy of activation distribution.
        Higher entropy = more uncertain/diverse representation
        """
        act = self.activations.get(layer_name)
        if act is None:
            return 0.0
        
        # Flatten and normalize to probability distribution
        flat = act.flatten()
        probs = np.abs(flat) / (np.abs(flat).sum() + 1e-10)
        
        return float(entropy(probs))
    
    def get_mean_activation(self, layer_name: str) -> float:
        """Get mean absolute activation value"""
        act = self.activations.get(layer_name)
        if act is None:
            return 0.0
        return float(np.abs(act).mean())
    
    def get_max_activation(self, layer_name: str) -> float:
        """Get maximum activation value"""
        act = self.activations.get(layer_name)
        if act is None:
            return 0.0
        return float(np.abs(act).max())
    
    def clear_activations(self):
        """Clear stored activations (free memory)"""
        self.activations.clear()
    
    def cleanup(self):
        """Remove all hooks and clear activations"""
        for hook in self.hooks:
            hook.remove()
        self.hooks.clear()
        self.clear_activations()
        print("✓ Hooks cleaned up")
    
    def get_summary(self) -> Dict:
        """Get summary statistics for all monitored layers"""
        summary = {}
        for layer_name in self.activations.keys():
            summary[layer_name] = {
                'entropy': self.get_entropy(layer_name),
                'mean': self.get_mean_activation(layer_name),
                'max': self.get_max_activation(layer_name),
                'shape': self.activations[layer_name].shape
            }
        return summary


# Example usage:
if __name__ == "__main__":
    from transformers import GPT2LMHeadModel, GPT2Tokenizer
    
    print("Loading GPT-2...")
    model = GPT2LMHeadModel.from_pretrained('gpt2')
    tokenizer = GPT2Tokenizer.from_pretrained('gpt2')
    
    # Monitor layers 6 and 11 (mid and late)
    monitor = ActivationMonitor(model, target_layers=[6, 11], model_type='gpt2')
    
    # Generate text
    prompt = "Hello, world!"
    inputs = tokenizer(prompt, return_tensors='pt')
    
    with torch.no_grad():
        outputs = model.generate(**inputs, max_new_tokens=10)
    
    # Check activations
    print("\nActivation Summary:")
    summary = monitor.get_summary()
    for layer, stats in summary.items():
        print(f"{layer}: entropy={stats['entropy']:.3f}, mean={stats['mean']:.3f}")
    
    monitor.cleanup()
