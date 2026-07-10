use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::GridEditorView;

/// The published `View` contract mirroring [`BuildMenuProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct BuildMenuView {
    pub config: Option<GridEditorView>,
}

impl ddd::View for BuildMenuView {}
