use super::view::OverrideKeyView;
use dioxus::prelude::*;

/// A single key-capture button in the hotkey-override section header. Clicking it activates
/// hotkey editing mode for the associated field.
#[derive(Props, Clone, PartialEq)]
pub struct OverrideKeyModel {
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
    /// Called when the player clicks to start editing.
    pub on_activate: EventHandler<()>,
}

impl From<&OverrideKeyView> for OverrideKeyModel {
    fn from(view: &OverrideKeyView) -> Self {
        let OverrideKeyView {
            label,
            is_editing,
            is_special,
            title,
            on_activate,
        } = view.clone();
        Self {
            label,
            is_editing,
            is_special,
            title,
            on_activate,
        }
    }
}

impl ddd::Model for OverrideKeyModel {
    type View = OverrideKeyView;
}
