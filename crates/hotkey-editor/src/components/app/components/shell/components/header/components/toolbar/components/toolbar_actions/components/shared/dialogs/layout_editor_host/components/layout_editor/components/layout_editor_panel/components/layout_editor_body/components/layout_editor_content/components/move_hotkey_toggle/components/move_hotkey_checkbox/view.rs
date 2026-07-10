use dioxus::prelude::*;

/// The published `View` contract mirroring [`MoveHotkeyCheckboxProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct MoveHotkeyCheckboxView {
    pub checked: bool,
    pub on_toggle: EventHandler<FormEvent>,
}

impl ddd::View for MoveHotkeyCheckboxView {}
