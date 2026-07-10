use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::GridEditorView;

/// The published `View` contract mirroring [`AltPositionPickerBodyProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AltPositionPickerBodyView {
    pub explainer_text: String,
    pub grid_config: GridEditorView,
}

impl ddd::View for AltPositionPickerBodyView {}
