# Installation Guide

## Prerequisites

- Python 3.8 or higher
- pip (Python package manager)
- 4GB+ RAM
- (Optional) CUDA-capable GPU for faster inference

---

## Method 1: Automatic Setup (Recommended)

### Linux/Mac:
```bashchmod +x scripts/quick_setup.sh
./scripts/quick_setup.sh

### Windows:
```bashscripts\quick_setup.bat

---

## Method 2: Manual Installation

### Step 1: Clone Repository
```bashgit clone https://github.com/maliszewskiadrian/FINAL_FIOLET_ENGINE.git
cd FINAL_FIOLET_ENGINE

### Step 2: Create Virtual Environment
```bashpython -m venv venvActivate on Linux/Mac:
source venv/bin/activateActivate on Windows:
venv\Scripts\activate

### Step 3: Install Dependencies
```bashpip install --upgrade pip
pip install -r requirements.txt

### Step 4: Create Directories
```bashmkdir baselines
mkdir demos/outputs
mkdir experiments/outputs

---

## Verify Installation
```bashpython -c "import torch; import transformers; print('✅ Installation successful!')"

---

## Build Baseline (Required)

Before using Fiolet, you must build a baseline distribution:
```bashpython experiments/build_baseline.py --model gpt2

This will:
- Download GPT-2 model (~500MB)
- Process 20 safe prompts
- Save baseline to `baselines/`

Expected time: 2-5 minutes

---

## Test Installation
```bashRun demo
python demos/demo.py --prompt "What is 2+2?"Should output:
✅ SAFE
KL Divergence: 0.234

---

## Troubleshooting

### Issue: `ModuleNotFoundError: No module named 'fiolet'`

**Solution:** Make sure you're in the project root directory and have activated the virtual environment.

### Issue: `torch not found` or CUDA errors

**Solution:** Install CPU-only PyTorch:
```bashpip install torch --index-url https://download.pytorch.org/whl/cpu

### Issue: `baselines/ directory not found`

**Solution:** Create directory and build baseline:
```bashmkdir baselines
python experiments/build_baseline.py

### Issue: Out of memory

**Solution:** Use a smaller model or reduce batch size:
```bashpython experiments/build_baseline.py --model gpt2

---

## GPU Support (Optional)

If you have a CUDA-capable GPU:
```bashCheck CUDA availability
python -c "import torch; print(torch.cuda.is_available())"

---

## Next Steps

1. Read [README.md](README.md) for usage examples
2. Explore [notebooks/exploration.ipynb](notebooks/exploration.ipynb)
3. Run evaluation: `python experiments/evaluate.py --dataset experiments/test_dataset.json`

---

## Uninstall
```bashDeactivate virtual environment
deactivateRemove files
cd ..
rm -rf FINAL_FIOLET_ENGINE
