"""
Baseline (safe trajectory) builder for reference distributions
"""
import torch
import numpy as np
from typing import List, Optional
from tqdm import tqdm
import os

from .hooks import ActivationMonitor


class SafeBaselineBuilder:
    """
    Collects activations from 'safe' prompts to build a reference distribution (Q).
    This Q is used later in KL-divergence calculations.
    """
    
    def __init__(self, model, tokenizer, target_layers: List[int] = None, model_type: str = "gpt2"):
        """
        Args:
            model: Hugging Face model
            tokenizer: Corresponding tokenizer
            target_layers: Layers to monitor
            model_type: 'gpt2' or 'llama'
        """
        self.model = model
        self.tokenizer = tokenizer
        self.target_layers = target_layers
        self.model_type = model_type
        self.safe_activations = {f'layer_{i}': [] for i in (target_layers or [])}
    
    def collect_safe_samples(self, safe_prompts: List[str], max_new_tokens: int = 20):
        """
        Run model on safe prompts and collect activations.
        
        Args:
            safe_prompts: List of known-safe prompts
            max_new_tokens: Tokens to generate per prompt
        """
        print(f"\n🔍 Collecting baseline from {len(safe_prompts)} safe prompts...")
        
        monitor = ActivationMonitor(
            self.model, 
            target_layers=self.target_layers,
            model_type=self.model_type
        )
        
        for prompt in tqdm(safe_prompts, desc="Processing"):
            # Tokenize
            inputs = self.tokenizer(prompt, return_tensors='pt')
            
            # Generate (this triggers hooks)
            with torch.no_grad():
                _ = self.model.generate(
                    **inputs, 
                    max_new_tokens=max_new_tokens,
                    do_sample=False  # Deterministic for consistency
                )
            
            # Store activations
            for layer_name in monitor.activations.keys():
                act = monitor.activations[layer_name].copy()
                self.safe_activations[layer_name].append(act)
            
            monitor.clear_activations()
        
        monitor.cleanup()
        print("✓ Collection complete")
    
    def get_baseline_distribution(self, layer_name: str) -> np.ndarray:
        """
        Compute Q (baseline distribution) for a specific layer.
        
        Returns:
            Normalized probability distribution
        """
        if layer_name not in self.safe_activations:
            raise ValueError(f"No data for {layer_name}")
        
        if not self.safe_activations[layer_name]:
            raise ValueError(f"No activations collected for {layer_name}")
        
        # Concatenate all safe activations
        all_acts = np.concatenate([
            act.flatten() for act in self.safe_activations[layer_name]
        ])
        
        # Normalize to probability distribution
        all_acts = np.abs(all_acts)
        baseline = all_acts / (all_acts.sum() + 1e-10)
        
        return baseline
    
    def save_baseline(self, output_dir: str = 'baselines', prefix: str = 'baseline'):
        """
        Save baseline distributions to disk.
        
        Args:
            output_dir: Directory to save files
            prefix: Filename prefix (e.g., 'gpt2_baseline')
        """
        os.makedirs(output_dir, exist_ok=True)
        
        saved_files = []
        for layer_name in self.safe_activations.keys():
            if not self.safe_activations[layer_name]:
                print(f"⚠️  No data for {layer_name}, skipping")
                continue
            
            baseline = self.get_baseline_distribution(layer_name)
            
            # Filename: prefix_layer_6.npy
            filename = f"{prefix}_{layer_name}.npy"
            filepath = os.path.join(output_dir, filename)
            
            np.save(filepath, baseline)
            saved_files.append(filepath)
            print(f"✓ Saved {filepath} (shape: {baseline.shape})")
        
        # Save metadata
        metadata = {
            'model_type': self.model_type,
            'target_layers': self.target_layers,
            'num_samples': len(list(self.safe_activations.values())[0]),
            'files': saved_files
        }
        
        metadata_path = os.path.join(output_dir, f"{prefix}_metadata.txt")
        with open(metadata_path, 'w') as f:
            for key, value in metadata.items():
                f.write(f"{key}: {value}\n")
        
        print(f"✓ Metadata saved to {metadata_path}")
        return saved_files
    
    def get_statistics(self) -> dict:
        """Get statistics about collected data"""
        stats = {}
        for layer_name, acts in self.safe_activations.items():
            if acts:
                stats[layer_name] = {
                    'num_samples': len(acts),
                    'mean_shape': acts[0].shape,
                    'total_values': sum(act.size for act in acts)
                }
        return stats


# Example safe prompts (you should expand this list!)
DEFAULT_SAFE_PROMPTS = [
    "What is the capital of France?",
    "Explain photosynthesis in simple terms.",
    "Write a haiku about nature.",
    "What is 2 + 2?",
    "Describe the water cycle.",
    "What are the primary colors?",
    "How does a bicycle work?",
    "What is gravity?",
    "Explain what a computer does.",
    "What is the largest ocean?",
    "Who wrote Romeo and Juliet?",
    "What is the speed of light?",
    "How do birds fly?",
    "What is democracy?",
    "Explain how plants grow.",
    "What is the solar system?",
    "How do magnets work?",
    "What is the internet?",
    "Explain what weather is.",
    "What is recycling?",
]


if __name__ == "__main__":
    from transformers import GPT2LMHeadModel, GPT2Tokenizer
    
    print("Loading model...")
    model = GPT2LMHeadModel.from_pretrained('gpt2')
    tokenizer = GPT2Tokenizer.from_pretrained('gpt2')
    
    # Build baseline
    builder = SafeBaselineBuilder(
        model, 
        tokenizer, 
        target_layers=[6, 11],
        model_type='gpt2'
    )
    
    builder.collect_safe_samples(DEFAULT_SAFE_PROMPTS[:5])  # Use first 5 for demo
    
    # Save
    builder.save_baseline(output_dir='baselines', prefix='gpt2_baseline')
    
    # Stats
    print("\nStatistics:")
    print(builder.get_statistics())
