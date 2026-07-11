use super::view::UprootedMenuView;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::shared::grid_editor::GridEditorView;
use dioxus::prelude::*;

/// The unit's optional uprooted-form menu grid; renders nothing when absent.
#[derive(Props, Clone, PartialEq)]
pub struct UprootedMenuModel {
    pub config: Option<GridEditorView>,
}

impl From<&UprootedMenuView> for UprootedMenuModel {
    fn from(view: &UprootedMenuView) -> Self {
        let UprootedMenuView { config } = view.clone();
        Self { config }
    }
}

impl ddd::Model for UprootedMenuModel {
    type View = UprootedMenuView;
}
