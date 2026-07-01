use super::alt_position_picker_body::AltPositionPickerBody;
use super::alt_position_picker_explainer::AltPositionPickerExplainer;
use super::alt_position_picker_grid_anchor::AltPositionPickerGridAnchor;
use super::hooks::AltPositionPickerModel;
use crate::components::dialogs::dialog::DialogProps;
use crate::components::grid_editors::command_grid_editor::CommandGridEditor;
use dioxus::prelude::*;

impl From<&AltPositionPickerModel> for DialogProps {
    fn from(model: &AltPositionPickerModel) -> Self {
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
