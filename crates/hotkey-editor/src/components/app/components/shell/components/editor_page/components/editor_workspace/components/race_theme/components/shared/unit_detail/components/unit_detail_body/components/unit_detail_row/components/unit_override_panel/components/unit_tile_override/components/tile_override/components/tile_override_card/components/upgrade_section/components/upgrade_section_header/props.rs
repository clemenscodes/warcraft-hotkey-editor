use super::components::upgrade_section_header_label_column::UpgradeSectionHeaderLabelColumnProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_override_panel::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::alt_state_label::AltStateLabelProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_override_panel::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::alt_state_position_button::AltStatePositionButtonProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_override_panel::components::unit_tile_override::components::tile_override::components::tile_override_card::components::shared::override_key::OverrideKeyProps;
use dioxus::prelude::*;

/// The upgraded-form block's top row: the label column beside the position button and
/// hotkey cell.
#[derive(Props, Clone, PartialEq)]
pub struct UpgradeSectionHeaderProps {
    pub label: AltStateLabelProps,
    pub position_button: AltStatePositionButtonProps,
    pub key_cell: OverrideKeyProps,
}

impl From<&UpgradeSectionHeaderProps> for UpgradeSectionHeaderLabelColumnProps {
    fn from(props: &UpgradeSectionHeaderProps) -> Self {
        let label = props.label.clone();
        Self { label }
    }
}

impl From<&UpgradeSectionHeaderProps> for AltStatePositionButtonProps {
    fn from(props: &UpgradeSectionHeaderProps) -> Self {
        props.position_button.clone()
    }
}

impl From<&UpgradeSectionHeaderProps> for OverrideKeyProps {
    fn from(props: &UpgradeSectionHeaderProps) -> Self {
        props.key_cell.clone()
    }
}
