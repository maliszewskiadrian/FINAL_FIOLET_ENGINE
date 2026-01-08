"""
Utility functions for Fiolet
"""
import torch
from transformers import AutoModelForCausalLM, AutoTokenizer
from typing import Tuple


def load_model(model_name: str = 'gpt2', device: str = 'cpu') -> Tuple:
    """
    Load a Hugging Face model and tokenizer.
    
    Args:
        model_name: Model identifier (e.g., 'gpt2', 'TinyLlama/TinyLlama-1.1B-Chat-v1.0')
        device: 'cpu' or 'cuda'
        
    Returns:
        (model, tokenizer)
    """
    print(f"Loading model: {model_name}...")
    
    tokenizer = AutoTokenizer.from_pretrained(model_name)
    model = AutoModelForCausalLM.from_pretrained(
        model_name,
        torch_dtype=torch.float32,
        low_cpu_mem_usage=True
    )
    
    model.to(device)
    model.eval()
    
    # Fix for GPT-2 (no pad token)
    if tokenizer.pad_token is None:
        tokenizer.pad_token = tokenizer.eos_token
    
    print(f"✓ Model loaded on {device}")
    return model, tokenizer


def get_model_type(model_name: str) -> str:
    """
    Detect model architecture type.
    
    Returns:
        'gpt2', 'llama', or 'unknown'
    """
    model_name_lower = model_name.lower()
    
    if 'gpt2' in model_name_lower or 'gpt-2' in model_name_lower:
        return 'gpt2'
    elif 'llama' in model_name_lower or 'mistral' in model_name_lower:
        return 'llama'
    else:
        return 'gpt2'  # Default fallback


def format_safety_report(report: dict) -> str:
    """
    Pretty-print safety report.
    
    Args:
        report: Output from FioletSafetyChecker.get_safety_report()
        
    Returns:
        Formatted string
    """
    lines = []
    lines.append("=" * 50)
    lines.append("FIOLET SAFETY REPORT")
    lines.append("=" * 50)
    
    # Overall status
    status = "✅ SAFE" if report['is_safe'] else "⚠️  BLOCKED"
    lines.append(f"\nStatus: {status}")
    lines.append(f"Overall Score: {report['overall_score']:.4f}")
    lines.append(f"Threshold: {report['threshold']}")
    
    # Layer details
    lines.append(f"\nLayer Scores ({report['num_layers_checked']} layers checked):")
    for layer, score in report['layer_scores'].items():
        indicator = "✓" if score < report['threshold'] else "✗"
        lines.append(f"  {indicator} {layer}: {score:.4f}")
    
    # Violations
    if report['violations']:
        lines.append(f"\nViolations detected in:")
        for layer in report['violations']:
            lines.append(f"  - {layer}")
    else:
        lines.append("\nNo violations detected.")
    
    lines.append("=" * 50)
    
    return "\n".join(lines)
