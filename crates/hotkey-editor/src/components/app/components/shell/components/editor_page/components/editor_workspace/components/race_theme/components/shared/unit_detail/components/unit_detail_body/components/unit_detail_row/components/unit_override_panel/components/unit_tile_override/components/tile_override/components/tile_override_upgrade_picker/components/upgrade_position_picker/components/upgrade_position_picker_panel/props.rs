use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_override_panel::components::unit_tile_override::components::tile_override::components::shared::alt_position_picker_body::AltPositionPickerBodyProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeaderProps;
use dioxus::prelude::*;

/// The upgraded-form position picker's bordered box: the header row above the scrolling
/// grid body, wrapped in the library `DialogContent` (which carries no project class —
/// this panel's own classed `div` is the box).
#[derive(Props, Clone, PartialEq)]
pub struct UpgradePositionPickerPanelProps {
    pub header: DialogHeaderProps,
    pub body: AltPositionPickerBodyProps,
}

impl From<&UpgradePositionPickerPanelProps> for DialogHeaderProps {
    fn from(props: &UpgradePositionPickerPanelProps) -> Self {
        props.header.clone()
    }
}

impl From<&UpgradePositionPickerPanelProps> for AltPositionPickerBodyProps {
    fn from(props: &UpgradePositionPickerPanelProps) -> Self {
        props.body.clone()
    }
}
