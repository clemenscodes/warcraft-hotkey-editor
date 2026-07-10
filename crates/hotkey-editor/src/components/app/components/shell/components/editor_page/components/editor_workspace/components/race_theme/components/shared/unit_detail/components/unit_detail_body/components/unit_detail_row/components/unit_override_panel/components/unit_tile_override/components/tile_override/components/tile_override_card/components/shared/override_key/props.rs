use crate::components::app::components::shell::components::shared::editable_keycap::{
    EditableKeycapProps, EditableKeycapState,
};
use dioxus::prelude::*;

/// A single key-capture button in the override panel header. Clicking it activates
/// hotkey editing mode for the associated field.
#[derive(Props, Clone, PartialEq)]
pub struct OverrideKeyProps {
    /// The visible label — a single letter, "–", "Esc", "Mouse4", etc.
    #[props(into)]
    pub label: String,
    /// Whether this cell is currently in capture/editing state.
    pub is_editing: bool,
    /// Whether the token is a non-letter special token (Esc, Mouse4, Mouse5); widens
    /// the cell for multi-character labels.
    pub is_special: bool,
    /// Human-readable title for the button (tooltip / accessibility).
    #[props(into)]
    pub title: String,
    /// Whether this is the card's primary key cell — the one a keyboard tile
    /// selection hands focus on to. Only the header hotkey cell sets this; the
    /// alt-state and upgrade cells (and the gallery previews) leave it false.
    #[props(default)]
    pub is_focus_target: bool,
    /// Called when the player clicks to start editing.
    pub on_activate: EventHandler<()>,
}

impl From<&OverrideKeyProps> for EditableKeycapProps {
    fn from(props: &OverrideKeyProps) -> Self {
        let label = props.label.clone();
        let state = if props.is_editing {
            EditableKeycapState::Editing
        } else {
            EditableKeycapState::Idle
        };
        Self { label, state }
    }
}
