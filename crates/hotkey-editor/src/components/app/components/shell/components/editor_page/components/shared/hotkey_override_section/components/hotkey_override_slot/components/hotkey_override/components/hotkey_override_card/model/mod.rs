use super::view::HotkeyOverrideCardView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

#[derive(Props, Clone, PartialEq)]
pub struct HotkeyOverrideCardModel {
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
    pub alt_name_text: Option<String>,
    pub show_alt_controls: bool,
    pub alt_hotkey_label: String,
    pub alt_hotkey_is_editing: bool,
    pub alt_hotkey_is_special_token: bool,
    pub on_hotkey_alt_position_click: EventHandler<()>,
    pub on_alt_hotkey_activate: EventHandler<()>,
    pub upgrade_show: bool,
    pub upgrade_hotkey_label: String,
    pub upgrade_is_editing: bool,
    pub upgrade_hotkey_is_special: bool,
    pub on_hotkey_upgrade_position_click: EventHandler<()>,
    pub on_upgrade_hotkey_activate: EventHandler<()>,
}

impl From<&HotkeyOverrideCardView> for HotkeyOverrideCardModel {
    fn from(view: &HotkeyOverrideCardView) -> Self {
        let HotkeyOverrideCardView {
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
            alt_name_text,
            show_alt_controls,
            alt_hotkey_label,
            alt_hotkey_is_editing,
            alt_hotkey_is_special_token,
            on_hotkey_alt_position_click,
            on_alt_hotkey_activate,
            upgrade_show,
            upgrade_hotkey_label,
            upgrade_is_editing,
            upgrade_hotkey_is_special,
            on_hotkey_upgrade_position_click,
            on_upgrade_hotkey_activate,
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
            alt_name_text,
            show_alt_controls,
            alt_hotkey_label,
            alt_hotkey_is_editing,
            alt_hotkey_is_special_token,
            on_hotkey_alt_position_click,
            on_alt_hotkey_activate,
            upgrade_show,
            upgrade_hotkey_label,
            upgrade_is_editing,
            upgrade_hotkey_is_special,
            on_hotkey_upgrade_position_click,
            on_upgrade_hotkey_activate,
        }
    }
}

impl ddd::Model for HotkeyOverrideCardModel {
    type View = HotkeyOverrideCardView;
}
