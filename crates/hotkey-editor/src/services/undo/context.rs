use dioxus::prelude::*;

use crate::services::undo::UndoHistory;

/// Access the [`UndoHistory`] provided at the app root. Call from a component or
/// hook body (it is a hook). Undo history is a global, crate-wide concept — no
/// single component owns it — so this accessor lives beside the type in
/// `services/`, not colocated with any component.
pub(crate) fn use_undo_history() -> UndoHistory {
    use_context()
}
