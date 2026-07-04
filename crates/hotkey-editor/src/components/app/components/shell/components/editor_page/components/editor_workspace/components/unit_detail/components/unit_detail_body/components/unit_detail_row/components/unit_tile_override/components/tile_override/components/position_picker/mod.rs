pub mod alt_position_picker_body;
pub mod alt_position_picker_explainer;
pub mod alt_position_picker_grid_anchor;
mod hooks;
mod logic;
mod props;
mod upgrade_position_picker;

use dioxus::prelude::*;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::dialog::{Dialog, DialogProps};
use hooks::use_alt_position_picker;

pub use props::AltPositionPickerProps;
pub use upgrade_position_picker::{UpgradePositionPicker, UpgradePositionPickerProps};

/// The off-state position picker dialog.
#[component]
pub fn AltPositionPicker(props: AltPositionPickerProps) -> Element {
    let model = use_alt_position_picker(&props);
    rsx! {
        Dialog { ..DialogProps::from(&model) }
    }
}
