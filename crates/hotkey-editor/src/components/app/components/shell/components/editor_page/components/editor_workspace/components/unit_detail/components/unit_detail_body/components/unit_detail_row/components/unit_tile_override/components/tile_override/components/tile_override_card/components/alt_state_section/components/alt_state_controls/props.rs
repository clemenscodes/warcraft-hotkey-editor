use dioxus::prelude::*;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::alt_state_position_button::AltStatePositionButtonProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::override_key_cell::OverrideKeyCellProps;

/// The editable controls of the off-state block: whether they show at all, plus the
/// off-state hotkey data and the two handlers.
#[derive(Props, Clone, PartialEq)]
pub struct AltStateControlsProps {
    pub show: bool,
    pub hotkey_label: String,
    pub is_editing: bool,
    pub is_special: bool,
    pub on_position_click: EventHandler<()>,
    pub on_hotkey_activate: EventHandler<()>,
}

impl From<&AltStateControlsProps> for AltStatePositionButtonProps {
    fn from(props: &AltStateControlsProps) -> Self {
        let title = String::from("Pick where the off-state button appears on the command card");
        Self {
            title,
            aria_label: "Edit off-state button position",
            on_click: props.on_position_click,
        }
    }
}

impl From<&AltStateControlsProps> for OverrideKeyCellProps {
    fn from(props: &AltStateControlsProps) -> Self {
        let label = props.hotkey_label.clone();
        let title = String::from("Hotkey for the off state (writes Unhotkey)");
        Self {
            label,
            is_editing: props.is_editing,
            is_special: props.is_special,
            title,
            is_focus_target: false,
            on_activate: props.on_hotkey_activate,
        }
    }
}
