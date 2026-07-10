use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::GridEditorView;
use dioxus::prelude::*;

/// The published `View` contract mirroring [`AltPositionPickerPanelProps`], threaded to this component as data.
#[derive(Clone, PartialEq)]
pub struct AltPositionPickerPanelView {
    pub title: String,
    pub on_close: EventHandler<()>,
    pub explainer_text: String,
    pub grid_config: GridEditorView,
}

impl ddd::View for AltPositionPickerPanelView {}
