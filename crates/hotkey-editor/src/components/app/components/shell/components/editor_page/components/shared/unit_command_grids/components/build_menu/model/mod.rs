use super::view::BuildMenuView;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::shared::grid_editor::GridEditorView;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BuildMenuModel {
    pub config: Option<GridEditorView>,
}

impl From<&BuildMenuView> for BuildMenuModel {
    fn from(view: &BuildMenuView) -> Self {
        let BuildMenuView { config } = view.clone();
        Self { config }
    }
}

impl ddd::Model for BuildMenuModel {
    type View = BuildMenuView;
}
