mod hooks;
mod props;
mod state;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use crate::components::dialogs::system_key_picker_dialog::SystemKeyPickerDialog;
use hooks::use_key_capture_cell;

pub use props::KeyCaptureCellProps;

assert_component!(KeyCaptureCell);

/// The key chip for a system-hotkey list row: shows the bound key and edits it on
/// click via the system key picker.
#[component]
pub fn KeyCaptureCell(props: KeyCaptureCellProps) -> Element {
    let model = use_key_capture_cell(&props);
    let class = style::class(model.state);
    rsx! {
        button {
            class,
            r#type: "button",
            "data-tooltip": model.conflict_title,
            "data-tooltip-placement": "above",
            onclick: model.on_click,
            {model.key_label}
        }
        if model.is_editing {
            SystemKeyPickerDialog {
                title: "Pick a hotkey",
                current_code: model.current_code,
                conflicts: model.picker_conflicts,
                open: true,
                on_pick: model.on_pick,
                on_close: model.on_close,
            }
        }
    }
}
