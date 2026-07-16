use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct UpgradeSectionHeaderView {
    pub hotkey_label: String,
    pub is_editing: bool,
    pub is_special: bool,
    pub on_position_click: EventHandler<()>,
    pub on_hotkey_activate: EventHandler<()>,
}

impl ddd::View for UpgradeSectionHeaderView {}
