use super::view::UpgradePositionPickerPanelView;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::GridEditorView;
use dioxus::prelude::*;

/// The upgraded-form position picker's bordered box: the header row above the scrolling
/// grid body, wrapped in the library `DialogContent` (which carries no project class —
/// this panel's own classed `div` is the box).
#[derive(Props, Clone, PartialEq)]
pub struct UpgradePositionPickerPanelProps {
    pub title: String,
    pub on_close: EventHandler<()>,
    pub explainer_text: String,
    pub grid_config: GridEditorView,
}

impl From<&UpgradePositionPickerPanelView> for UpgradePositionPickerPanelProps {
    fn from(view: &UpgradePositionPickerPanelView) -> Self {
        let UpgradePositionPickerPanelView {
            title,
            on_close,
            explainer_text,
            grid_config,
        } = view.clone();
        Self {
            title,
            on_close,
            explainer_text,
            grid_config,
        }
    }
}

impl ddd::Props for UpgradePositionPickerPanelProps {
    type View = UpgradePositionPickerPanelView;
}
