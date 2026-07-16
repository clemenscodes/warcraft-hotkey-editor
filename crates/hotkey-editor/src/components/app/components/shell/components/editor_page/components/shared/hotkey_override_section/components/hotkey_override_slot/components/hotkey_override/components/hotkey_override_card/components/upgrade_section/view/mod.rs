use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct UpgradeSectionView {
    pub show: bool,
    pub upgrade_hotkey_label: String,
    pub upgrade_is_editing: bool,
    pub upgrade_hotkey_is_special: bool,
    pub on_position_click: EventHandler<()>,
    pub on_hotkey_activate: EventHandler<()>,
}

impl ddd::View for UpgradeSectionView {}
