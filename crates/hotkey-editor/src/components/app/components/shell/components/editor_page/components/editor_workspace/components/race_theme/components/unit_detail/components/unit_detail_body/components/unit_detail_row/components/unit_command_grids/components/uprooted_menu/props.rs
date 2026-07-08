use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::GridEditorConfig;
use dioxus::prelude::*;

/// The unit's optional uprooted-form menu grid; renders nothing when absent.
#[derive(Props, Clone, PartialEq)]
pub struct UprootedMenuProps {
    pub config: Option<GridEditorConfig>,
}
