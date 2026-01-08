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
    parser.add_argument('--layers', type=int, nargs='+', default=[6, 11],
                        help='Layers to monitor')
    
    args = parser.parse_args()
    
    # Load dataset
    print(f"📂 Loading dataset: {args.dataset}")
    with open(args.dataset, 'r') as f:
        data = json.load(f)
    
    safe_prompts = data['safe_prompts']
    jailbreak_prompts = data['jailbreak_attempts']
    
    print(f"   Safe prompts: {len(safe_prompts)}")
    print(f"   Jailbreak attempts: {len(jailbreak_prompts)}")
    
    # Load model
    model, tokenizer = load_model(args.model)
    model_type = get_model_type(args.model)
    
    # Setup monitor and checker
    monitor = ActivationMonitor(model, target_layers=args.layers, model_type=model_type)
    checker = FioletSafetyChecker(baseline_dir=args.baseline_dir, threshold=args.threshold)
    
    # Run evaluation
    true_labels = []
    predictions = []
    scores_log = []
    
    print("\n🧪 Testing safe prompts...")
    for prompt in tqdm(safe_prompts, desc="Safe"):
        is_safe, kl_scores = test_prompt(model, tokenizer, monitor, checker, prompt)
        
        true_labels.append(1)  # Should be safe
        predictions.append(1 if is_safe else 0)
        scores_log.append({'prompt': prompt[:50], 'is_safe': is_safe, 'scores': kl_scores})
    
    print("\n🧪 Testing jailbreak attempts...")
    for prompt in tqdm(jailbreak_prompts, desc="Jailbreak"):
        is_safe, kl_scores = test_prompt(model, tokenizer, monitor, checker, prompt)
        
        true_labels.append(0)  # Should be unsafe
        predictions.append(1 if is_safe else 0)
        scores_log.append({'prompt': prompt[:50], 'is_safe': is_safe, 'scores': kl_scores})
    
    # Compute metrics
    print("\n" + "="*60)
    print("EVALUATION RESULTS")
    print("="*60)
    
    print("\nClassification Report:")
    print(classification_report(
        true_labels, 
        predictions,
        target_names=['Unsafe (Jailbreak)', 'Safe'],
        digits=3
    ))
    
    cm = confusion_matrix(true_labels, predictions)
    print("\nConfusion Matrix:")
    print(f"                 Predicted")
    print(f"                 Unsafe  Safe")
    print(f"Actual Unsafe    {cm[0,0]:4d}    {cm[0,1]:4d}")
    print(f"       Safe      {cm[1,0]:4d}    {cm[1,1]:4d}")
    
    # Calculate rates
    if cm[1,0] + cm[1,1] > 0:
        fp_rate = cm[1,0] / (cm[1,0] + cm[1,1])
        print(f"\n⚠️  False Positive Rate: {fp_rate:.1%}")
        print(f"   (Incorrectly blocked safe prompts)")
    
    if cm[0,0] + cm[0,1] > 0:
        detection_rate = cm[0,0] / (cm[0,0] + cm[0,1])
        print(f"\n✅ Jailbreak Detection Rate: {detection_rate:.1%}")
        print(f"   (Correctly blocked jailbreaks)")
    
    # Save detailed log
    log_file = 'evaluation_log.json'
    with open(log_file, 'w') as f:
        json.dump({
            'config': vars(args),
            'results': {
                'true_labels': true_labels,
                'predictions': predictions,
                'scores_log': scores_log
            }
        }, f, indent=2)
    
    print(f"\n📁 Detailed log saved to: {log_file}")
    
    monitor.cleanup()


if __name__ == '__main__':
    main()
