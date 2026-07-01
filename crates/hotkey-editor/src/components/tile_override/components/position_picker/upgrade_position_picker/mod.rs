mod hooks;
mod props;

use dioxus::prelude::*;

use super::alt_position_picker_body::AltPositionPickerBody;

use super::alt_position_picker_explainer::{
    AltPositionPickerExplainer, AltPositionPickerExplainerProps,
};

use super::alt_position_picker_grid_anchor::AltPositionPickerGridAnchor;
use crate::components::dialogs::dialog::Dialog;
use crate::components::grid_editors::command_grid_editor::CommandGridEditor;
use hooks::use_upgrade_position_picker;

pub use props::UpgradePositionPickerProps;

/// The upgraded-form position picker dialog.
#[component]
pub fn UpgradePositionPicker(props: UpgradePositionPickerProps) -> Element {
    let model = use_upgrade_position_picker(&props);
    let explainer = AltPositionPickerExplainerProps::from(&props);
    let open = props.upgrade_position_picker_open;
    rsx! {
        Dialog {
            open,
            title: model.dialog_title,
            AltPositionPickerBody {
                AltPositionPickerExplainer { ..explainer }
                AltPositionPickerGridAnchor {
                    CommandGridEditor { ..model.grid_config }
                }
            }
        }
    }
}
