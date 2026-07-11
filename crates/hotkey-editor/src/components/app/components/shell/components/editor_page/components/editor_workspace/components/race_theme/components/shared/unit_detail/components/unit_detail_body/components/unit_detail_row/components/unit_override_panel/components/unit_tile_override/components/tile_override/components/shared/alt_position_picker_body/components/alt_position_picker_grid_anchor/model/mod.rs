use super::view::AltPositionPickerGridAnchorView;
use dioxus::prelude::*;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::shared::grid_editor::GridEditorView;

/// The grid anchor's input: the grid config for the embedded command grid.
#[derive(Props, Clone, PartialEq)]
pub struct AltPositionPickerGridAnchorModel {
    pub grid_config: GridEditorView,
}

impl From<&AltPositionPickerGridAnchorModel> for GridEditorView {
    fn from(props: &AltPositionPickerGridAnchorModel) -> Self {
        props.grid_config.clone()
    }
}

impl From<&AltPositionPickerGridAnchorView> for AltPositionPickerGridAnchorModel {
    fn from(view: &AltPositionPickerGridAnchorView) -> Self {
        let AltPositionPickerGridAnchorView { grid_config } = view.clone();
        Self { grid_config }
    }
}

impl ddd::Model for AltPositionPickerGridAnchorModel {
    type View = AltPositionPickerGridAnchorView;
}
