use super::super::alt_position_picker_body::AltPositionPickerBody;
use super::super::alt_position_picker_explainer::AltPositionPickerExplainer;
use super::super::alt_position_picker_grid_anchor::AltPositionPickerGridAnchor;
use super::hooks::UpgradePositionPickerModel;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::dialog::DialogProps;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::unit_detail::components::unit_detail_body::components::unit_detail_row::components::shared::grid_editors::command_grid_editor::CommandGridEditor;
use dioxus::prelude::*;

impl From<&UpgradePositionPickerModel> for DialogProps {
    fn from(model: &UpgradePositionPickerModel) -> Self {
        let open = model.open;
        let title = model.dialog_title.clone();
        let explainer = model.explainer.clone();
        let grid_config = model.grid_config.clone();
        let children = rsx! {
            AltPositionPickerBody {
                AltPositionPickerExplainer { ..explainer }
                AltPositionPickerGridAnchor {
                    CommandGridEditor { ..grid_config }
                }
            }
        };
        Self {
            open,
            title,
            children,
            footer: None,
            on_open_change: None,
        }
    }
}
