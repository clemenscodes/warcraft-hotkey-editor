mod model;
mod view;

pub use view::HotkeyPickerDialogHostDialogView;

use dioxus::prelude::*;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker_dialog::KeyPickerDialog;
use tw_macro::assert_component;

use model::HotkeyPickerDialogHostDialogModel;

/// Mounts the hotkey picker only while a hotkey field is being edited.
#[component]
pub fn HotkeyPickerDialogHost(props: HotkeyPickerDialogHostDialogModel) -> Element {
    if !props.visible {
        return rsx! {};
    }
    let HotkeyPickerDialogHostDialogModel {
        title,
        rows,
        on_pick,
        on_close,
        ..
    } = props;
    rsx! {
        KeyPickerDialog {
            title,
            rows,
            open: true,
            allow_conflict_pick: false,
            on_pick,
            on_close,
        }
    }
}

assert_component!(HotkeyPickerDialogHost);
