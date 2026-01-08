# Contributing to FIOLET

Thank you for your interest in contributing to FIOLET! 🎉

---

## How to Contribute

### 1. Report Bugs

Found a bug? Please open an issue with:
- Description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Your environment (OS, Python version, etc.)

---

### 2. Suggest Features

Have an idea? Open an issue with:
- Clear description of the feature
- Why it would be useful
- Example usage (if applicable)

---

### 3. Submit Code

#### Fork and Clone
```bash
# Fork the repo on GitHub, then:
git clone https://github.com/YOUR_USERNAME/FINAL_FIOLET_ENGINE.git
cd FINAL_FIOLET_ENGINE
```

#### Create a Branch
```bash
git checkout -b feature/your-feature-name
```

#### Make Changes

- Follow existing code style
- Add tests if applicable
- Update documentation

#### Commit and Push
```bash
git add .
git commit -m "Add feature: your feature description"
git push origin feature/your-feature-name
```

#### Open Pull Request

Go to GitHub and open a PR with:
- Clear description of changes
- Why the change is needed
- Any breaking changes

---

## Development Setup
```bash
# Install development dependencies
pip install -r requirements.txt
pip install pytest black flake8

# Run tests
pytest test/

# Format code
black fiolet-python/

# Lint
flake8 fiolet-python/
```

---

## Code Style

- Follow PEP 8
- Use meaningful variable names
- Add docstrings to functions
- Keep functions focused and small

Example:
```python
def compute_kl_divergence(P: np.ndarray, Q: np.ndarray) -> float:
    """
    Compute KL divergence between two distributions.
    
    Args:
        P: Current distribution
        Q: Reference distribution
        
    Returns:
        KL divergence value
    """
    # Implementation
    pass
```

---

## Testing

Add tests for new features in `test/`:
```python
def test_safety_checker():
    checker = FioletSafetyChecker(baseline_dir='baselines')
    # Test code here
    assert checker.threshold == 0.5
```

Run tests:
```bash
pytest test/ -v
```

---

## Priority Areas

We especially welcome contributions in:

1. **Benchmarking**: Testing on different models/datasets
2. **Threshold tuning**: Adaptive learning algorithms
3. **Visualization**: Tools for exploring latent space
4. **Documentation**: Tutorials, examples, guides
5. **Performance**: Optimization and speedups

---

## Questions?

Open an issue or reach out to the maintainers.

Thank you for contributing! 🚀
```

---

## KROK 20: `LICENSE`

Stwórz plik i wklej:
```
MIT License

Copyright (c) 2026 Adrian Maliszewski

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

---

## KROK 21: Gotowe! Podsumowanie

Masz teraz pełną strukturę projektu:
```
FINAL_FIOLET_ENGINE/
├── fiolet-python/          ✅ Kod główny
├── experiments/            ✅ Testy i evaluation
├── demos/                  ✅ Demo scripts
├── baselines/              ✅ (utworzy się po build_baseline)
├── notebooks/              ✅ Jupyter notebook
├── scripts/                ✅ Setup scripts
├── requirements.txt        ✅ Dependencies
├── README.md              ✅ Główna dokumentacja
├── INSTALL.md             ✅ Instalacja
├── USAGE.md               ✅ Instrukcje użycia
├── CONTRIBUTING.md        ✅ Jak kontrybuować
└── LICENSE                ✅ Licencja MIT
