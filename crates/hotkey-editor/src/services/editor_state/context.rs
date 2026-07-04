use dioxus::prelude::*;

use crate::services::editor_state::EditorState;

/// Access the [`EditorState`] provided by the app shell. Call from a component or
/// hook body (it is a hook). The editor's UI state is a shell-wide concept the
/// editor page reads from context rather than as props, so this accessor lives
/// beside the type in `services/`, not colocated with any component.
pub(crate) fn use_editor_state() -> EditorState {
    use_context::<EditorState>()
}
