use super::view::ResearchMenuView;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::shared::grid_editor::GridEditorView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ResearchMenuModel {
    pub config: Option<GridEditorView>,
}

impl From<&ResearchMenuView> for ResearchMenuModel {
    fn from(view: &ResearchMenuView) -> Self {
        let ResearchMenuView { config } = view.clone();
        Self { config }
    }
}

impl ddd::Model for ResearchMenuModel {
    type View = ResearchMenuView;
}
