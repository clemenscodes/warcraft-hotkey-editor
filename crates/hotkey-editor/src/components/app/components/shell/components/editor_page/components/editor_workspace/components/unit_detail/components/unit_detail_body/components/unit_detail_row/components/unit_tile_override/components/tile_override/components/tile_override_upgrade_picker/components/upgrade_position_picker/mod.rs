mod hooks;
mod logic;
mod props;

use dioxus::prelude::*;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::dialog::{Dialog, DialogProps};
use hooks::use_upgrade_position_picker;

pub use props::UpgradePositionPickerProps;

/// The upgraded-form position picker dialog.
use tw_macro::assert_component;
assert_component!(UpgradePositionPicker);
#[component]
pub fn UpgradePositionPicker(props: UpgradePositionPickerProps) -> Element {
    let model = use_upgrade_position_picker(&props);
    rsx! {
        Dialog { ..DialogProps::from(&model) }
    }
}
