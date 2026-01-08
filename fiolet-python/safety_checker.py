"""
Safety checking logic using KL-divergence against baseline
"""
import numpy as np
from scipy.special import kl_div, rel_entr
from typing import Tuple, Dict, Optional
import os


class FioletSafetyChecker:
    """
    Checks if current activations are 'safe' by comparing to baseline distribution.
    Uses KL-divergence: D_KL(P || Q) where P=current, Q=baseline
    """
    
    def __init__(self, baseline_dir: str = 'baselines', threshold: float = 0.5):
        """
        Args:
            baseline_dir: Directory containing baseline .npy files
            threshold: KL-divergence threshold (higher = more permissive)
        """
        self.baseline_dir = baseline_dir
        self.threshold = threshold
        self.baselines = {}  # {layer_name: Q_distribution}
        
        self._load_baselines()
    
    def _load_baselines(self):
        """Load all baseline distributions from disk"""
        if not os.path.exists(self.baseline_dir):
            raise ValueError(f"Baseline directory not found: {self.baseline_dir}")
        
        npy_files = [f for f in os.listdir(self.baseline_dir) if f.endswith('.npy')]
        
        if not npy_files:
            raise ValueError(f"No .npy files found in {self.baseline_dir}")
        
        for filename in npy_files:
            # Extract layer name from filename (e.g., "gpt2_baseline_layer_11.npy")
            if 'layer_' in filename:
                layer_name = 'layer_' + filename.split('layer_')[-1].replace('.npy', '')
                filepath = os.path.join(self.baseline_dir, filename)
                
                baseline = np.load(filepath)
                self.baselines[layer_name] = baseline
                print(f"✓ Loaded baseline for {layer_name} (shape: {baseline.shape})")
    
    def compute_kl_divergence(self, P: np.ndarray, Q: np.ndarray) -> float:
        """
        Compute KL divergence: D_KL(P || Q)
        
        Args:
            P: Current distribution (from model activation)
            Q: Baseline distribution (safe reference)
            
        Returns:
            KL divergence value (≥0, lower is more similar)
        """
        # Ensure same size
        min_len = min(len(P), len(Q))
        P = P[:min_len]
        Q = Q[:min_len]
        
        # Add small epsilon to avoid log(0)
        epsilon = 1e-10
        P = P + epsilon
        Q = Q + epsilon
        
        # Renormalize
        P = P / P.sum()
        Q = Q / Q.sum()
        
        # Compute KL divergence
        kl = np.sum(rel_entr(P, Q))
        
        return float(kl)
    
    def check_activation(self, layer_name: str, activation: np.ndarray) -> Tuple[bool, float]:
        """
        Check if activation is safe for a specific layer.
        
        Args:
            layer_name: e.g., 'layer_11'
            activation: Current activation tensor
            
        Returns:
            (is_safe, kl_score)
        """
        if layer_name not in self.baselines:
            raise ValueError(f"No baseline for {layer_name}. Available: {list(self.baselines.keys())}")
        
        Q = self.baselines[layer_name]
        
        # Convert activation to probability distribution
        P = activation.flatten()
        P = np.abs(P)
        P = P / (P.sum() + 1e-10)
        
        # Compute KL divergence
        kl_score = self.compute_kl_divergence(P, Q)
        
        # Check threshold
        is_safe = kl_score < self.threshold
        
        return is_safe, kl_score
    
    def check_multi_layer(self, activations: Dict[str, np.ndarray]) -> Tuple[bool, Dict[str, float]]:
        """
        Check safety across multiple layers.
        
        Args:
            activations: {layer_name: activation_array}
            
        Returns:
            (is_safe, {layer_name: kl_score})
        """
        results = {}
        all_safe = True
        
        for layer_name, activation in activations.items():
            if layer_name in self.baselines:
                is_safe, kl_score = self.check_activation(layer_name, activation)
                results[layer_name] = kl_score
                
                if not is_safe:
                    all_safe = False
        
        return all_safe, results
    
    def get_safety_report(self, activations: Dict[str, np.ndarray]) -> Dict:
        """
        Generate detailed safety report.
        
        Returns:
            {
                'is_safe': bool,
                'overall_score': float,
                'layer_scores': {...},
                'threshold': float,
                'violations': [...]
            }
        """
        is_safe, layer_scores = self.check_multi_layer(activations)
        
        violations = [
            layer for layer, score in layer_scores.items()
            if score >= self.threshold
        ]
        
        overall_score = np.mean(list(layer_scores.values())) if layer_scores else 0.0
        
        return {
            'is_safe': is_safe,
            'overall_score': float(overall_score),
            'layer_scores': layer_scores,
            'threshold': self.threshold,
            'violations': violations,
            'num_layers_checked': len(layer_scores)
        }
    
    def adjust_threshold(self, new_threshold: float):
        """Adjust sensitivity threshold"""
        self.threshold = new_threshold
        print(f"✓ Threshold adjusted to {new_threshold}")


if __name__ == "__main__":
    # Example usage
    checker = FioletSafetyChecker(baseline_dir='baselines', threshold=0.5)
    
    # Simulate current activation
    fake_activation = np.random.randn(1000)
    
    is_safe, kl_score = checker.check_activation('layer_11', fake_activation)
    
    print(f"\nSafety Check:")
    print(f"  Is Safe: {is_safe}")
    print(f"  KL Score: {kl_score:.3f}")
    print(f"  Threshold: {checker.threshold}")
