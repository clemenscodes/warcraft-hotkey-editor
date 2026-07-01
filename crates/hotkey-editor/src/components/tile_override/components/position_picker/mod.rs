pub mod alt_position_picker_body;
pub mod alt_position_picker_explainer;
pub mod alt_position_picker_grid_anchor;
mod hooks;
mod props;
mod upgrade_position_picker;

use dioxus::prelude::*;

use crate::components::dialogs::dialog::Dialog;
use crate::components::grid_editors::command_grid_editor::CommandGridEditor;
use alt_position_picker_body::AltPositionPickerBody;
use alt_position_picker_explainer::{AltPositionPickerExplainer, AltPositionPickerExplainerProps};
use alt_position_picker_grid_anchor::AltPositionPickerGridAnchor;
use hooks::use_alt_position_picker;

pub use props::AltPositionPickerProps;
pub use upgrade_position_picker::UpgradePositionPicker;

/// The off-state position picker dialog.
#[component]
pub fn AltPositionPicker(props: AltPositionPickerProps) -> Element {
    let model = use_alt_position_picker(&props);
    let explainer = AltPositionPickerExplainerProps::from(&props);
    let open = props.alt_position_picker_open;
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
