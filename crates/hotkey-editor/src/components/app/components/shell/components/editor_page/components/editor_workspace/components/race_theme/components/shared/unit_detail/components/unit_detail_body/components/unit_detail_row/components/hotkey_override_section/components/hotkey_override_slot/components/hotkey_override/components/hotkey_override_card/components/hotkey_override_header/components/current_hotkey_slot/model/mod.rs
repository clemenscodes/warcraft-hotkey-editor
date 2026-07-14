use super::view::CurrentHotkeySlotView;
use dioxus::prelude::*;

/// The right side of the override header: exactly one of the hotkey cell, the
/// research hotkey cell, or the passive note (or nothing), chosen by the flags.
#[derive(Props, Clone, PartialEq)]
pub struct CurrentHotkeySlotModel {
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

impl From<&CurrentHotkeySlotView> for CurrentHotkeySlotModel {
    fn from(view: &CurrentHotkeySlotView) -> Self {
        let CurrentHotkeySlotView {
            show_hotkey_field,
            hotkey_label,
            hotkey_is_editing,
            hotkey_is_special,
            on_hotkey_activate,
            show_research_field,
            research_label,
            research_is_editing,
            research_is_special,
            on_research_activate,
            is_info_only,
        } = view.clone();
        Self {
            show_hotkey_field,
            hotkey_label,
            hotkey_is_editing,
            hotkey_is_special,
            on_hotkey_activate,
            show_research_field,
            research_label,
            research_is_editing,
            research_is_special,
            on_research_activate,
            is_info_only,
        }
    }
}

impl ddd::Model for CurrentHotkeySlotModel {
    type View = CurrentHotkeySlotView;
}
