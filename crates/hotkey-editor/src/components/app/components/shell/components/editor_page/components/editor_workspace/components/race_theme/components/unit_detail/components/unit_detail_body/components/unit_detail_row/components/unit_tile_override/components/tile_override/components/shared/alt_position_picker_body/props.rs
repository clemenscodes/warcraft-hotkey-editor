use dioxus::prelude::*;

use super::components::alt_position_picker_explainer::AltPositionPickerExplainerProps;
use super::components::alt_position_picker_grid_anchor::AltPositionPickerGridAnchorProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::GridEditorConfig;

/// The picker body's inputs: the explainer copy and the grid config for the
/// embedded command grid.
#[derive(Props, Clone, PartialEq)]
pub struct AltPositionPickerBodyProps {
    pub explainer: AltPositionPickerExplainerProps,
    pub grid_config: GridEditorConfig,
}

impl From<&AltPositionPickerBodyProps> for AltPositionPickerExplainerProps {
    fn from(props: &AltPositionPickerBodyProps) -> Self {
        props.explainer.clone()
    }
}

impl From<&AltPositionPickerBodyProps> for AltPositionPickerGridAnchorProps {
    fn from(props: &AltPositionPickerBodyProps) -> Self {
        let grid_config = props.grid_config.clone();
        Self { grid_config }
    }
}
