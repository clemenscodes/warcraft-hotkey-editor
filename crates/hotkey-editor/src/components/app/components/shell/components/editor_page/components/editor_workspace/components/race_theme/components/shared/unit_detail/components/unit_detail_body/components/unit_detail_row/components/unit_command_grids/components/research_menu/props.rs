use super::view::ResearchMenuView;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::GridEditorView;
use dioxus::prelude::*;

/// The unit's optional research menu grid; renders nothing when absent.
#[derive(Props, Clone, PartialEq)]
pub struct ResearchMenuProps {
    pub config: Option<GridEditorView>,
}

impl From<&ResearchMenuView> for ResearchMenuProps {
    fn from(view: &ResearchMenuView) -> Self {
        let ResearchMenuView { config } = view.clone();
        Self { config }
    }
}

impl ddd::Props for ResearchMenuProps {
    type View = ResearchMenuView;
}
