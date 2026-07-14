use super::view::UpgradeSectionHeaderView;
use dioxus::prelude::*;

/// The upgraded-form block's top row: the label column beside the position button and
/// hotkey cell.
#[derive(Props, Clone, PartialEq)]
pub struct UpgradeSectionHeaderModel {
    pub hotkey_label: String,
    pub is_editing: bool,
    pub is_special: bool,
    pub on_position_click: EventHandler<()>,
    pub on_hotkey_activate: EventHandler<()>,
}

impl From<&UpgradeSectionHeaderView> for UpgradeSectionHeaderModel {
    fn from(view: &UpgradeSectionHeaderView) -> Self {
        let UpgradeSectionHeaderView {
            hotkey_label,
            is_editing,
            is_special,
            on_position_click,
            on_hotkey_activate,
        } = view.clone();
        Self {
            hotkey_label,
            is_editing,
            is_special,
            on_position_click,
            on_hotkey_activate,
        }
    }
}

impl ddd::Model for UpgradeSectionHeaderModel {
    type View = UpgradeSectionHeaderView;
}
