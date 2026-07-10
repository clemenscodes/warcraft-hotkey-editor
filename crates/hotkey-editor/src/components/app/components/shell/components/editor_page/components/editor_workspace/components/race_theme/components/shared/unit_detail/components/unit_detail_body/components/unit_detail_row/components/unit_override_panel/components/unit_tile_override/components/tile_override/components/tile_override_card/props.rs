use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The override card owns the header and the four ability sections; every field its
/// children place is threaded through here as a domain value by the panel's hook.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideCardProps {
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
    pub description_lines: Vec<String>,
    pub alt_name_text: Option<String>,
    pub alt_description_lines: Vec<String>,
    pub show_alt_controls: bool,
    pub alt_hotkey_label: String,
    pub alt_hotkey_is_editing: bool,
    pub alt_hotkey_is_special_token: bool,
    pub on_alt_position_click: EventHandler<()>,
    pub on_alt_hotkey_activate: EventHandler<()>,
    pub upgrade_show: bool,
    pub upgrade_hotkey_label: String,
    pub upgrade_is_editing: bool,
    pub upgrade_hotkey_is_special: bool,
    pub on_upgrade_position_click: EventHandler<()>,
    pub on_upgrade_hotkey_activate: EventHandler<()>,
    pub active_tier_index: usize,
    pub total_tier_count: usize,
    pub tier_label_text: String,
}
