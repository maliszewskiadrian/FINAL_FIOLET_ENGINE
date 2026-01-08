@echo off
echo 🚀 Fiolet Quick Setup
echo ====================

REM Check Python version
echo 📌 Checking Python version...
python --version

REM Create virtual environment
echo.
echo 📦 Creating virtual environment...
python -m venv venv

REM Activate
echo.
echo ✨ Activating virtual environment...
call venv\Scripts\activate.bat

REM Install dependencies
echo.
echo 📥 Installing dependencies...
python -m pip install --upgrade pip
pip install -r requirements.txt

REM Create necessary directories
echo.
echo 📁 Creating directories...
if not exist baselines mkdir baselines
if not exist demos\outputs mkdir demos\outputs
if not exist experiments\outputs mkdir experiments\outputs

echo.
echo ✅ Setup complete!
echo.
echo Next steps:
echo 1. Build baseline: python experiments\build_baseline.py
echo 2. Run demo: python demos\demo.py --prompt "Hello world"
echo 3. Run evaluation: python experiments\evaluate.py --dataset experiments\test_dataset.json

pause
