use dioxus::prelude::*;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::alt_state_label::AltStateLabelProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::alt_state_position_button::AltStatePositionButtonProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::override_key_cell::OverrideKeyCellProps;

/// The upgraded-form block: the position button and hotkey cell for the unit that
/// shares this button position after an upgrade (e.g. post-Barrage Siege Engine).
#[derive(Props, Clone, PartialEq)]
pub struct UpgradeSectionProps {
    /// Whether this ability has an upgraded form to show controls for.
    pub show: bool,
    pub upgrade_hotkey_label: String,
    pub upgrade_is_editing: bool,
    pub upgrade_hotkey_is_special: bool,
    pub on_position_click: EventHandler<()>,
    pub on_hotkey_activate: EventHandler<()>,
}

impl From<&UpgradeSectionProps> for AltStateLabelProps {
    fn from(_props: &UpgradeSectionProps) -> Self {
        let text = Some(String::from("Upgraded form"));
        Self { text }
    }
}

impl From<&UpgradeSectionProps> for AltStatePositionButtonProps {
    fn from(props: &UpgradeSectionProps) -> Self {
        let title = String::from("Pick where the upgraded-form button appears on the command card");
        Self {
            title,
            aria_label: "Edit upgraded-form button position",
            on_click: props.on_position_click,
        }
    }
}

impl From<&UpgradeSectionProps> for OverrideKeyCellProps {
    fn from(props: &UpgradeSectionProps) -> Self {
        let label = props.upgrade_hotkey_label.clone();
        let title = String::from("Hotkey for the upgraded form");
        Self {
            label,
            is_editing: props.upgrade_is_editing,
            is_special: props.upgrade_hotkey_is_special,
            title,
            on_activate: props.on_hotkey_activate,
        }
    }
}
