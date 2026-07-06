mod hooks;
mod logic;
mod props;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::dialog::{Dialog, DialogProps};
use dioxus::prelude::*;
use hooks::use_alt_position_picker;
pub use props::AltPositionPickerProps;
use tw_macro::assert_component;
assert_component!(AltPositionPicker);

/// The off-state position picker dialog.
#[component]
pub fn AltPositionPicker(props: AltPositionPickerProps) -> Element {
    let model = use_alt_position_picker(&props);
    rsx! {
        Dialog { ..DialogProps::from(&model) }
    }
}
