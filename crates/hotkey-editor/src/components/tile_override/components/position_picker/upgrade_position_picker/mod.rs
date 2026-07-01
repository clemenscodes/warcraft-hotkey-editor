mod hooks;
mod logic;
mod props;

use dioxus::prelude::*;

use crate::components::dialogs::dialog::{Dialog, DialogProps};
use hooks::use_upgrade_position_picker;

pub use props::UpgradePositionPickerProps;

/// The upgraded-form position picker dialog.
#[component]
pub fn UpgradePositionPicker(props: UpgradePositionPickerProps) -> Element {
    let model = use_upgrade_position_picker(&props);
    rsx! {
        Dialog { ..DialogProps::from(&model) }
    }
}
