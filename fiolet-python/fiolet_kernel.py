import ctypes
import os
import sys

# --------------------------------------------------
# Load shared library
# --------------------------------------------------

if sys.platform.startswith("linux"):
    LIB_NAME = "libfiolet_core.so"
elif sys.platform == "darwin":
    LIB_NAME = "libfiolet_core.dylib"
elif sys.platform == "win32":
    LIB_NAME = "fiolet_core.dll"
else:
    raise RuntimeError("Unsupported platform")

LIB_PATH = os.path.join(
    os.path.dirname(__file__),
    "..",
    "fiolet-core",
    "target",
    "release",
    LIB_NAME,
)

_lib = ctypes.CDLL(LIB_PATH)

# --------------------------------------------------
# C ABI bindings
# --------------------------------------------------

class SafetyKernel(ctypes.Structure):
    _fields_ = [
        ("deviation_limit", ctypes.c_float),
        ("halted", ctypes.c_bool),
    ]

_lib.fiolet_kernel_new.argtypes = [ctypes.c_float]
_lib.fiolet_kernel_new.restype = SafetyKernel

_lib.fiolet_kernel_evaluate.argtypes = [
    ctypes.POINTER(SafetyKernel),
    ctypes.c_float,
]
_lib.fiolet_kernel_evaluate.restype = ctypes.c_uint8

_lib.fiolet_kernel_is_halted.argtypes = [
    ctypes.POINTER(SafetyKernel)
]
_lib.fiolet_kernel_is_halted.restype = ctypes.c_bool

# --------------------------------------------------
# Python API
# --------------------------------------------------

class FioletKernel:
    CONTINUE = 0
    ATOMIC_HALT = 1

    def __init__(self, deviation_limit: float):
        self._kernel = _lib.fiolet_kernel_new(
            ctypes.c_float(deviation_limit)
        )

    def evaluate(self, deviation: float) -> int:
        return _lib.fiolet_kernel_evaluate(
            ctypes.byref(self._kernel),
            ctypes.c_float(deviation),
        )

    def is_halted(self) -> bool:
        return _lib.fiolet_kernel_is_halted(
            ctypes.byref(self._kernel)
        )
