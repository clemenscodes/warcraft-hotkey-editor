use dioxus::prelude::*;

/// The published `View` contract mirroring [`OverrideKeyProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct OverrideKeyView {
    /// The visible label — a single letter, "–", "Esc", "Mouse4", etc.
    pub label: String,
    /// Whether this cell is currently in capture/editing state.
    pub is_editing: bool,
    /// Whether the token is a non-letter special token (Esc, Mouse4, Mouse5); widens
    /// the cell for multi-character labels.
    pub is_special: bool,
    /// Human-readable title for the button (tooltip / accessibility).
    pub title: String,
    /// Called when the player clicks to start editing.
    pub on_activate: EventHandler<()>,
}

impl ddd::View for OverrideKeyView {}
