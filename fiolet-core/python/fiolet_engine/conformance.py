"""
Read-only FIOLET conformance binding.

This module does NOT:
- generate output
- explain HALT
- recover from failure
"""

class Halt(Exception):
    """Raised when FIOLET enforces HALT."""
    pass


def validate_trace(trace: dict) -> None:
    """
    Validate an epistemic trace via FINAL_FIOLET_ENGINE.

    If HALT is triggered:
        - raises Halt
    If allowed:
        - returns None

    There is no return value on success.
    Silence is correctness.
    """

    # This function is a thin boundary.
    # The actual enforcement happens in Rust via the adapter.
    #
    # Python NEVER decides.

    decision = _engine_validate(trace)

    if decision == "HALT":
        raise Halt()


def _engine_validate(trace: dict) -> str:
    """
    Placeholder for Rust FFI call.

    This MUST call:
        FioletConformanceAdapter::validate(...)
    """

    # IMPORTANT:
    # This function is intentionally opaque.
    # Implementation is via FFI only.
    raise NotImplementedError("FFI binding not linked")
