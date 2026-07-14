use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::shared::grid_editor::GridEditorView;

#[derive(Clone, PartialEq)]
pub struct BuildMenuView {
    pub config: Option<GridEditorView>,
}

impl ddd::View for BuildMenuView {}
