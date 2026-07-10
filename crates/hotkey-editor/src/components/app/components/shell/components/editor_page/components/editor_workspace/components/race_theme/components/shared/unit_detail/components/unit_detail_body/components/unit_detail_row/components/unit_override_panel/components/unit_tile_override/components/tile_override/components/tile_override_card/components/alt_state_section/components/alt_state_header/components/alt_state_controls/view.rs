use dioxus::prelude::*;

/// The published `View` contract mirroring [`AltStateControlsProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AltStateControlsView {
    pub show: bool,
    pub hotkey_label: String,
    pub is_editing: bool,
    pub is_special: bool,
    pub on_position_click: EventHandler<()>,
    pub on_hotkey_activate: EventHandler<()>,
}

impl ddd::View for AltStateControlsView {}
