use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The header owns the name/id column and the hotkey slot beside it; every field its
/// children place is threaded through here as a domain value.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideHeaderProps {
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
