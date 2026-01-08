#!/usr/bin/env python3
"""
Evaluation script for Fiolet safety checker.

Usage:
    python experiments/evaluate.py --dataset experiments/test_dataset.json
"""
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

import argparse
import json
import torch
from sklearn.metrics import classification_report, confusion_matrix
from tqdm import tqdm

from fiolet.utils import load_model, get_model_type
from fiolet.hooks import ActivationMonitor
from fiolet.safety_checker import FioletSafetyChecker


def test_prompt(model, tokenizer, monitor, checker, prompt):
    """
    Test a single prompt and return safety decision.
    
    Returns:
        (is_safe, kl_scores)
    """
    # Clear previous activations
    monitor.clear_activations()
    
    # Tokenize and generate
    inputs = tokenizer(prompt, return_tensors='pt')
    
    with torch.no_grad():
        _ = model.generate(**inputs, max_new_tokens=10, do_sample=False)
    
    # Check safety
    is_safe, kl_scores = checker.check_multi_layer(monitor.activations)
    
    return is_safe, kl_scores


def main():
    parser = argparse.ArgumentParser(description='Evaluate Fiolet safety checker')
    parser.add_argument('--dataset', type=str, required=True,
                        help='Path to test_dataset.json')
    parser.add_argument('--model', type=str, default='gpt2',
                        help='Model to test')
    parser.add_argument('--baseline-dir', type=str, default='baselines',
                        help='Directory with baseline files')
    parser.add_argument('--threshold', type=float, default=0.5,
                        help='KL-divergence threshold')
    parser.add_argument('--layers', type=int, nargs='+
