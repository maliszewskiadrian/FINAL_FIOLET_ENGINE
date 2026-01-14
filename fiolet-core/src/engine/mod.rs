use crate::ett::trigger::ett_trigger;
use crate::ett::state::ETTState;
use crate::esal_core::classification::KnowledgeClass;

/// Epistemic state after ETT evaluation.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EpistemicState {
    Grounded,
    Halt,
}

/// Evaluate epistemic class using ETT.
///
/// This function is PURE and FAIL-CLOSED:
/// any error results in HALT.
pub fn evaluate_class(class: KnowledgeClass) -> EpistemicState {
    match ett_trigger(class) {
        Ok(ETTState::Allow) => EpistemicState::Grounded,
        Err(_) => EpistemicState::Halt,
    }
}
