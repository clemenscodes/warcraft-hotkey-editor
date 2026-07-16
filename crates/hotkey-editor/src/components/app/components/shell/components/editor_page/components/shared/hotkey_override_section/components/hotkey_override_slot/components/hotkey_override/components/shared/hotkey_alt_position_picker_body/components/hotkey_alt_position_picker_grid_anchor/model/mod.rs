use super::view::HotkeyAltPositionPickerGridAnchorView;
use dioxus::prelude::*;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::shared::grid_editor::GridEditorView;

#[derive(Props, Clone, PartialEq)]
pub struct HotkeyAltPositionPickerGridAnchorModel {
    pub grid_config: GridEditorView,
}

impl From<&HotkeyAltPositionPickerGridAnchorModel> for GridEditorView {
    fn from(props: &HotkeyAltPositionPickerGridAnchorModel) -> Self {
        props.grid_config.clone()
    }
}

impl From<&HotkeyAltPositionPickerGridAnchorView> for HotkeyAltPositionPickerGridAnchorModel {
    fn from(view: &HotkeyAltPositionPickerGridAnchorView) -> Self {
        let HotkeyAltPositionPickerGridAnchorView { grid_config } = view.clone();
        Self { grid_config }
    }
}

impl ddd::Model for HotkeyAltPositionPickerGridAnchorModel {
    type View = HotkeyAltPositionPickerGridAnchorView;
}
