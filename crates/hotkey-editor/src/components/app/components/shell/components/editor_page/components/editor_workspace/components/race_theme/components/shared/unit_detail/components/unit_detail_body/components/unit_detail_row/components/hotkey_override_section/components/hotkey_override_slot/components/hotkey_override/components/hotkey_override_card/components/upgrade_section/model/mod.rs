use super::view::UpgradeSectionView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct UpgradeSectionModel {
    pub show: bool,
    pub upgrade_hotkey_label: String,
    pub upgrade_is_editing: bool,
    pub upgrade_hotkey_is_special: bool,
    pub on_position_click: EventHandler<()>,
    pub on_hotkey_activate: EventHandler<()>,
}

impl From<&UpgradeSectionView> for UpgradeSectionModel {
    fn from(view: &UpgradeSectionView) -> Self {
        let UpgradeSectionView {
            show,
            upgrade_hotkey_label,
            upgrade_is_editing,
            upgrade_hotkey_is_special,
            on_position_click,
            on_hotkey_activate,
        } = view.clone();
        Self {
            show,
            upgrade_hotkey_label,
            upgrade_is_editing,
            upgrade_hotkey_is_special,
            on_position_click,
            on_hotkey_activate,
        }
    }
}

impl ddd::Model for UpgradeSectionModel {
    type View = UpgradeSectionView;
}
