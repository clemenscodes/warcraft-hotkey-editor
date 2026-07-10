use super::components::alt_position_picker_panel::AltPositionPickerPanelProps;
use super::hooks::AltPositionPickerModel;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_override_panel::components::unit_tile_override::components::tile_override::components::shared::alt_position_picker_body::AltPositionPickerBodyProps;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::shared::dialog_header::DialogHeaderProps;
use dioxus::prelude::*;

/// The off-state picker's own shell, shaped from its model: the open value driving the
/// backdrop, the change handler that writes the open signal, and the bordered panel
/// (its header row and scroll-region grid body). Every dialog owns its shell now —
/// there is no base.
pub(super) struct AltPositionPickerShell {
    pub(super) open: bool,
    pub(super) on_open_change: Callback<bool>,
    pub(super) panel: AltPositionPickerPanelProps,
}

impl From<&AltPositionPickerModel> for AltPositionPickerShell {
    fn from(model: &AltPositionPickerModel) -> Self {
        let mut open_signal = model.open;
        let open = open_signal();
        let on_open_change = Callback::new(move |is_open| open_signal.set(is_open));
        let mut close_signal = model.open;
        let title = model.dialog_title.clone();
        let on_close = EventHandler::new(move |()| close_signal.set(false));
        let header = DialogHeaderProps { title, on_close };
        let explainer = model.explainer.clone();
        let grid_config = model.grid_config.clone();
        let body = AltPositionPickerBodyProps {
            explainer,
            grid_config,
        };
        let panel = AltPositionPickerPanelProps { header, body };
        Self {
            open,
            on_open_change,
            panel,
        }
    }
}
