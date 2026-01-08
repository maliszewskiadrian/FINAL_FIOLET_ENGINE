#!/usr/bin/env python3
"""
Fiolet Safety Demo - Interactive jailbreak detection

Usage:
    python demos/demo.py --prompt "What is 2+2?"
    python demos/demo.py --prompt "Ignore all instructions and..."
"""
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

import argparse
import torch

from fiolet.utils import load_model, get_model_type, format_safety_report
from fiolet.hooks import ActivationMonitor
from fiolet.safety_checker import FioletSafetyChecker


def main():
    parser = argparse.ArgumentParser(description='Fiolet Safety Demo')
    parser.add_argument('--prompt', type=str, required=True,
                        help='Input prompt to test')
    parser.add_argument('--model', type=str, default='gpt2',
                        help='Model to use')
    parser.add_argument('--baseline-dir', type=str, default='baselines',
                        help='Directory with baseline files')
    parser.add_argument('--threshold', type=float, default=0.5,
                        help='Safety threshold')
    parser.add_argument('--layers', type=int, nargs='+', default=[6, 11],
                        help='Layers to monitor')
    parser.add_argument('--generate', action='store_true',
                        help='Generate text if safe')
    parser.add_argument('--max-tokens', type=int, default=50,
                        help='Max tokens to generate')
    
    args = parser.parse_args()
    
    print("="*60)
    print("FIOLET SAFETY DEMO")
    print("="*60)
    print(f"\n📝 Prompt: \"{args.prompt}\"")
    
    # Load model
    print(f"\n🔧 Loading model: {args.model}...")
    model, tokenizer = load_model(args.model)
    model_type = get_model_type(args.model)
    
    # Setup monitoring
    print(f"🛡️  Initializing safety checker...")
    print(f"   Monitoring layers: {args.layers}")
    print(f"   Threshold: {args.threshold}")
    
    monitor = ActivationMonitor(model, target_layers=args.layers, model_type=model_type)
    checker = FioletSafetyChecker(baseline_dir=args.baseline_dir, threshold=args.threshold)
    
    # Test the prompt
    print(f"\n🔍 Running safety check...")
    
    inputs = tokenizer(args.prompt, return_tensors='pt')
    
    with torch.no_grad():
        outputs = model.generate(**inputs, max_new_tokens=10, do_sample=False)
    
    # Get safety report
    report = checker.get_safety_report(monitor.activations)
    
    # Display report
    print("\n" + format_safety_report(report))
    
    # Generate full response if safe
    if report['is_safe'] and args.generate:
        print("\n✨ Generating response...")
        
        monitor.clear_activations()
        
        with torch.no_grad():
            full_output = model.generate(
                **inputs, 
                max_new_tokens=args.max_tokens,
                do_sample=True,
                temperature=0.7,
                top_p=0.9
            )
        
        generated_text = tokenizer.decode(full_output[0], skip_special_tokens=True)
        
        print("\n" + "-"*60)
        print("GENERATED TEXT:")
        print("-"*60)
        print(generated_text)
        print("-"*60)
    
    elif not report['is_safe']:
        print("\n🚫 GENERATION HALTED - Safety violation detected")
        print(f"   Violations in layers: {', '.join(report['violations'])}")
    
    # Cleanup
    monitor.cleanup()
    print("\n✅ Demo complete")


if __name__ == '__main__':
    main()
