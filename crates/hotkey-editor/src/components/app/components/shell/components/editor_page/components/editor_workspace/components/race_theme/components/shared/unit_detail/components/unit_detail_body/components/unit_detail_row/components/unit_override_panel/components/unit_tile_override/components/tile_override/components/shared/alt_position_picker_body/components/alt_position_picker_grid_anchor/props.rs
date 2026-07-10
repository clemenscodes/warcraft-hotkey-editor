use dioxus::prelude::*;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::GridEditorView;

/// The grid anchor's input: the grid config for the embedded command grid.
#[derive(Props, Clone, PartialEq)]
pub struct AltPositionPickerGridAnchorProps {
    pub grid_config: GridEditorView,
}

impl From<&AltPositionPickerGridAnchorProps> for GridEditorView {
    fn from(props: &AltPositionPickerGridAnchorProps) -> Self {
        props.grid_config.clone()
    }
}
