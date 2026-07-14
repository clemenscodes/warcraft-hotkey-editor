use dioxus::prelude::*;

/// The published `View` contract mirroring [`CurrentHotkeySlotModel`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct CurrentHotkeySlotView {
    pub show_hotkey_field: bool,
    pub hotkey_label: String,
    pub hotkey_is_editing: bool,
    pub hotkey_is_special: bool,
    pub on_hotkey_activate: EventHandler<()>,
    pub show_research_field: bool,
    pub research_label: String,
    pub research_is_editing: bool,
    pub research_is_special: bool,
    pub on_research_activate: EventHandler<()>,
    pub is_info_only: bool,
}

impl ddd::View for CurrentHotkeySlotView {}
