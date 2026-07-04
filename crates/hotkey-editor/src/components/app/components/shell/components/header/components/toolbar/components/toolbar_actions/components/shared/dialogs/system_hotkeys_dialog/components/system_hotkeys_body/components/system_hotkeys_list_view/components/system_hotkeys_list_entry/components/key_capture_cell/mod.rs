mod hooks;
mod logic;
mod props;
mod state;
mod style;

use crate::assert_component;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_key_picker_dialog::{
    SystemKeyPickerDialog, SystemKeyPickerDialogProps,
};
use dioxus::prelude::*;
use hooks::use_key_capture_cell;
pub use props::KeyCaptureCellProps;
assert_component!(KeyCaptureCell);

/// The key chip for a system-hotkey list row: shows the bound key and edits it on
/// click via the system key picker.
#[component]
pub fn KeyCaptureCell(props: KeyCaptureCellProps) -> Element {
    let model = use_key_capture_cell(&props);
    let picker = SystemKeyPickerDialogProps::from(&model);
    let class = style::class(model.state);
    rsx! {
        button {
            class,
            r#type: "button",
            "data-tooltip": model.conflict_title,
            onclick: model.on_click,
            {model.key_label}
        }
        if model.is_editing {
            SystemKeyPickerDialog { ..picker }
        }
    }
}
