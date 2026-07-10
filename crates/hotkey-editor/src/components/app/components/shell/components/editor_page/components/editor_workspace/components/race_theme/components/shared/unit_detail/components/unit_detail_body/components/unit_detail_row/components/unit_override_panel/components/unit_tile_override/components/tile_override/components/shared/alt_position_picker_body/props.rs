use dioxus::prelude::*;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::GridEditorConfig;

/// The picker body's inputs: the explainer copy and the grid config for the
/// embedded command grid.
#[derive(Props, Clone, PartialEq)]
pub struct AltPositionPickerBodyProps {
    pub explainer_text: String,
    pub grid_config: GridEditorConfig,
}
