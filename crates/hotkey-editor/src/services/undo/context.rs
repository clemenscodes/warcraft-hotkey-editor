use dioxus::prelude::*;

use crate::services::undo::UndoHistory;

pub(crate) fn use_undo_history() -> UndoHistory {
    use_context()
}
