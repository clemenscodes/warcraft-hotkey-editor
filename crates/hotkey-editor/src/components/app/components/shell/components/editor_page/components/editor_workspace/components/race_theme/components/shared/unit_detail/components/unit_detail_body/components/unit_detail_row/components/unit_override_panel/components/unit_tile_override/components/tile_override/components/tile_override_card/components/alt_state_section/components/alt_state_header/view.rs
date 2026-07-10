use dioxus::prelude::*;

/// The published `View` contract mirroring [`AltStateHeaderProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AltStateHeaderView {
    pub alt_name_text: Option<String>,
    pub show: bool,
    pub hotkey_label: String,
    pub is_editing: bool,
    pub is_special: bool,
    pub on_position_click: EventHandler<()>,
    pub on_hotkey_activate: EventHandler<()>,
}

impl ddd::View for AltStateHeaderView {}
