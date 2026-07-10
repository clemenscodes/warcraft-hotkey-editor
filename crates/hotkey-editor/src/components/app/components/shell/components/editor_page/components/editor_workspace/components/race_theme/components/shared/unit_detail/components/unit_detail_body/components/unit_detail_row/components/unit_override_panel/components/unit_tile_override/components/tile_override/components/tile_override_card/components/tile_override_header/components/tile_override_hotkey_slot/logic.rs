use super::components::tile_override_info_only::TileOverrideInfoOnlyProps;
use super::props::TileOverrideHotkeySlotProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_override_panel::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::override_key::OverrideKeyProps;

/// Which of the three mutually-exclusive slot contents applies: the hotkey cell,
/// the research-hotkey cell, the passive note, or nothing.
pub(super) struct TileOverrideHotkeySlotDispatch {
    pub(super) key_cell: Option<OverrideKeyProps>,
    pub(super) info: Option<TileOverrideInfoOnlyProps>,
}

impl From<&TileOverrideHotkeySlotProps> for TileOverrideHotkeySlotDispatch {
    fn from(props: &TileOverrideHotkeySlotProps) -> Self {
        if props.show_hotkey_field {
            let key_cell = OverrideKeyProps {
                label: props.hotkey_label.clone(),
                is_editing: props.hotkey_is_editing,
                is_special: props.hotkey_is_special,
                title: String::from("Hotkey"),
                is_focus_target: true,
                on_activate: props.on_hotkey_activate,
            };
            return Self {
                key_cell: Some(key_cell),
                info: None,
            };
        }
        if props.show_research_field {
            let key_cell = OverrideKeyProps {
                label: props.research_label.clone(),
                is_editing: props.research_is_editing,
                is_special: props.research_is_special,
                title: String::from("Research hotkey"),
                is_focus_target: true,
                on_activate: props.on_research_activate,
            };
            return Self {
                key_cell: Some(key_cell),
                info: None,
            };
        }
        if props.is_info_only {
            let info = TileOverrideInfoOnlyProps {
                text: String::from("Passive racial ability"),
            };
            return Self {
                key_cell: None,
                info: Some(info),
            };
        }
        Self {
            key_cell: None,
            info: None,
        }
    }
}
