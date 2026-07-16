use super::view::HotkeyOverrideHeaderView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct HotkeyOverrideHeaderModel {
    pub name_text: String,
    pub object_id: WarcraftObjectId,
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

impl From<&HotkeyOverrideHeaderView> for HotkeyOverrideHeaderModel {
    fn from(view: &HotkeyOverrideHeaderView) -> Self {
        let HotkeyOverrideHeaderView {
            name_text,
            object_id,
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
            name_text,
            object_id,
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

impl ddd::Model for HotkeyOverrideHeaderModel {
    type View = HotkeyOverrideHeaderView;
}
