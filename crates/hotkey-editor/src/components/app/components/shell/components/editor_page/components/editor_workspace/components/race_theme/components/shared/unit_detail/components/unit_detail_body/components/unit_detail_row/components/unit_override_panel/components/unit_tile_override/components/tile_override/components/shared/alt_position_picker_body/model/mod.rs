use super::view::AltPositionPickerBodyView;
use dioxus::prelude::*;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::shared::grid_editor::GridEditorView;

/// The picker body's inputs: the explainer copy and the grid config for the
/// embedded command grid.
#[derive(Props, Clone, PartialEq)]
pub struct AltPositionPickerBodyModel {
    pub explainer_text: String,
    pub grid_config: GridEditorView,
}

impl From<&AltPositionPickerBodyView> for AltPositionPickerBodyModel {
    fn from(view: &AltPositionPickerBodyView) -> Self {
        let AltPositionPickerBodyView {
            explainer_text,
            grid_config,
        } = view.clone();
        Self {
            explainer_text,
            grid_config,
        }
    }
}

impl ddd::Model for AltPositionPickerBodyModel {
    type View = AltPositionPickerBodyView;
}
