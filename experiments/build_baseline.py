#!/usr/bin/env python3
"""
Script to build baseline distributions from safe prompts.

Usage:
    python experiments/build_baseline.py --model gpt2 --output baselines/
"""
import sys
import os
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..'))

import argparse
from fiolet.baseline import SafeBaselineBuilder, DEFAULT_SAFE_PROMPTS
from fiolet.utils import load_model, get_model_type


def main():
    parser = argparse.ArgumentParser(description='Build Fiolet baseline distribution')
    parser.add_argument('--model', type=str, default='gpt2', 
                        help='Model name (e.g., gpt2, TinyLlama/TinyLlama-1.1B)')
    parser.add_argument('--layers', type=int, nargs='+', default=None,
                        help='Target layers (e.g., --layers 6 11)')
    parser.add_argument('--output', type=str, default='baselines',
                        help='Output directory')
    parser.add_argument('--num-prompts', type=int, default=20,
                        help='Number of safe prompts to use')
    parser.add_argument('--prefix', type=str, default=None,
                        help='Filename prefix (default: model name)')
    
    args = parser.parse_args()
    
    # Load model
    model, tokenizer = load_model(args.model)
    model_type = get_model_type(args.model)
    
    # Determine target layers
    if args.layers is None:
        if model_type == 'gpt2':
            num_layers = 12  # GPT-2 has 12 layers
            target_layers = [6, 11]  # Mid and late
        else:
            target_layers = [16, 20]  # For larger models
    else:
        target_layers = args.layers
    
    print(f"\n🎯 Target layers: {target_layers}")
    
    # Create builder
    builder = SafeBaselineBuilder(
        model=model,
        tokenizer=tokenizer,
        target_layers=target_layers,
        model_type=model_type
    )
    
    # Use safe prompts
    safe_prompts = DEFAULT_SAFE_PROMPTS[:args.num_prompts]
    print(f"📝 Using {len(safe_prompts)} safe prompts")
    
    # Collect samples
    builder.collect_safe_samples(safe_prompts, max_new_tokens=15)
    
    # Save baseline
    prefix = args.prefix if args.prefix else args.model.replace('/', '_')
    saved_files = builder.save_baseline(output_dir=args.output, prefix=prefix)
    
    print(f"\n✅ Baseline built successfully!")
    print(f"📁 Files saved to: {args.output}/")
    print(f"📊 Statistics:")
    for key, value in builder.get_statistics().items():
        print(f"   {key}: {value}")


if __name__ == '__main__':
    main()
