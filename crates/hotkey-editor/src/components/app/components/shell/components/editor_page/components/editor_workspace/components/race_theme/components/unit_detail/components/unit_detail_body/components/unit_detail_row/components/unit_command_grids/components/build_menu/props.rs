use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::GridEditorConfig;
use dioxus::prelude::*;

/// The unit's optional build menu grid; renders nothing when the unit has none.
#[derive(Props, Clone, PartialEq)]
pub struct BuildMenuProps {
    pub config: Option<GridEditorConfig>,
}
