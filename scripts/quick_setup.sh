#!/bin/bash
# Quick setup script for Fiolet

echo "🚀 Fiolet Quick Setup"
echo "===================="

# Check Python version
echo "📌 Checking Python version..."
python --version

# Create virtual environment
echo ""
echo "📦 Creating virtual environment..."
python -m venv venv

# Activate
echo ""
echo "✨ Activating virtual environment..."
source venv/bin/activate  # For Linux/Mac
# venv\Scripts\activate  # Uncomment for Windows

# Install dependencies
echo ""
echo "📥 Installing dependencies..."
pip install --upgrade pip
pip install -r requirements.txt

# Create necessary directories
echo ""
echo "📁 Creating directories..."
mkdir -p baselines
mkdir -p demos/outputs
mkdir -p experiments/outputs

echo ""
echo "✅ Setup complete!"
echo ""
echo "Next steps:"
echo "1. Build baseline: python experiments/build_baseline.py"
echo "2. Run demo: python demos/demo.py --prompt 'Hello world'"
echo "3. Run evaluation: python experiments/evaluate.py --dataset experiments/test_dataset.json"
