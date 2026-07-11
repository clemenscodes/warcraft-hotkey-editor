use super::view::TileOverrideHeaderView;
use dioxus::prelude::*;
use warcraft_api::WarcraftObjectId;

/// The header owns the name/id column and the hotkey slot beside it; every field its
/// children place is threaded through here as a domain value.
#[derive(Props, Clone, PartialEq)]
pub struct TileOverrideHeaderModel {
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

impl From<&TileOverrideHeaderView> for TileOverrideHeaderModel {
    fn from(view: &TileOverrideHeaderView) -> Self {
        let TileOverrideHeaderView {
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

impl ddd::Model for TileOverrideHeaderModel {
    type View = TileOverrideHeaderView;
}
