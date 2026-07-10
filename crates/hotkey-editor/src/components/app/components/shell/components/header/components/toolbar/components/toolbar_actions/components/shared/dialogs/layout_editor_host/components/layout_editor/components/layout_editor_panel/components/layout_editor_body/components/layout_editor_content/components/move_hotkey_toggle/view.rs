use dioxus::prelude::*;

/// The published `View` contract mirroring [`MoveHotkeyToggleProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MoveHotkeyToggleView {
    pub checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
}

impl ddd::View for MoveHotkeyToggleView {}
