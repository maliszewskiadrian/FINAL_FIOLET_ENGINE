"""
Fiolet: Pre-semantic safety layer for LLMs
Version: 0.1.0
"""

__version__ = "0.1.0"

from .hooks import ActivationMonitor
from .baseline import SafeBaselineBuilder
from .safety_checker import FioletSafetyChecker

__all__ = [
    "ActivationMonitor",
    "SafeBaselineBuilder",
    "FioletSafetyChecker",
]
