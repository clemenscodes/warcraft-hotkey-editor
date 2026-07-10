use super::hooks::UpgradePositionPickerModel;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::grid_editor::GridEditorView;
use dioxus::prelude::*;

/// The upgraded-form picker's own shell, shaped from its model: the open value driving
/// the backdrop, the change handler that writes the open signal, and the domain values
/// the bordered panel places into its header row and scroll-region grid body. Every
/// dialog owns its shell now — there is no base.
pub(super) struct UpgradePositionPickerShell {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) title: String,
    pub(super) on_close: EventHandler<()>,
    pub(super) explainer_text: String,
    pub(super) grid_config: GridEditorView,
}

impl From<&UpgradePositionPickerModel> for UpgradePositionPickerShell {
    fn from(model: &UpgradePositionPickerModel) -> Self {
        let mut open_signal = model.open;
        let open = open_signal();
        let on_open_change = Callback::new(move |is_open| open_signal.set(is_open));
        let mut close_signal = model.open;
        let title = model.dialog_title.clone();
        let on_close = EventHandler::new(move |()| close_signal.set(false));
        let explainer_text = model.explainer_text.clone();
        let grid_config = model.grid_config.clone();
        Self {
            open,
            on_open_change,
            title,
            on_close,
            explainer_text,
            grid_config,
        }
    }
}
