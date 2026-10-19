mod model;
mod view;

pub use view::HotkeyPickerDialogDialogView;

use dioxus::prelude::*;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::key_picker_dialog::KeyPickerDialog;
use tw_macro::assert_component;

use model::HotkeyPickerDialogDialogModel;

#[component]
pub fn HotkeyPickerDialog(props: HotkeyPickerDialogDialogModel) -> Element {
    if !props.visible {
        return rsx! {};
    }
    let HotkeyPickerDialogDialogModel {
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

assert_component!(HotkeyPickerDialog);
