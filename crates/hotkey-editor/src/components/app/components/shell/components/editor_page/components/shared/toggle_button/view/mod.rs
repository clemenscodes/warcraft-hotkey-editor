use dioxus::prelude::*;

/// The published `View` contract mirroring [`ToggleButtonModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct ToggleButtonView {
    /// The button text.
    pub label: &'static str,
    /// Whether this button is the active one in its group.
    pub active: bool,
    /// An optional tooltip (the catalog toggles explain what they widen; the mode and
    /// search toggles have none).
    pub title: Option<&'static str>,
    /// Activation handler.
    pub onclick: EventHandler<MouseEvent>,
    /// Extra keyboard handling on top of the native button (the mode toggle moves
    /// focus onto the race tabs); the others leave it at the no-op default.
    pub onkeydown: EventHandler<KeyboardEvent>,
}

impl ddd::View for ToggleButtonView {}
