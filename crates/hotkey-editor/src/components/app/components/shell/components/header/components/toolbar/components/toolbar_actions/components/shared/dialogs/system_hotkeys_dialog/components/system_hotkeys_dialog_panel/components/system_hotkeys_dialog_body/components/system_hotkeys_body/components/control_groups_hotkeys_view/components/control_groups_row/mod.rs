mod hooks;
mod props;
mod style;

use tw_macro::assert_component;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::slot_button::SlotButton;
use dioxus::prelude::*;
use hooks::use_control_groups_row;
pub use props::ControlGroupsRowProps;
use style::CLASS;
assert_component!(ControlGroupsRow);

/// The ten-cell control-group strip.
#[component]
pub fn ControlGroupsRow(props: ControlGroupsRowProps) -> Element {
    let model = use_control_groups_row(&props);
    rsx! {
        div {
            class: CLASS,
            style: model.frame,
            for slot in model.slots {
                SlotButton { ..slot }
            }
        }
    }
}
