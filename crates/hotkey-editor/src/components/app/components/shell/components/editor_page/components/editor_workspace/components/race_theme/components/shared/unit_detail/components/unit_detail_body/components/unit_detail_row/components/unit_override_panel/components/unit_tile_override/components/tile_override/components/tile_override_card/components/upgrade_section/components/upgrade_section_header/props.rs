use dioxus::prelude::*;

/// The upgraded-form block's top row: the label column beside the position button and
/// hotkey cell.
#[derive(Props, Clone, PartialEq)]
pub struct UpgradeSectionHeaderProps {
    pub hotkey_label: String,
    pub is_editing: bool,
    pub is_special: bool,
    pub on_position_click: EventHandler<()>,
    pub on_hotkey_activate: EventHandler<()>,
}
