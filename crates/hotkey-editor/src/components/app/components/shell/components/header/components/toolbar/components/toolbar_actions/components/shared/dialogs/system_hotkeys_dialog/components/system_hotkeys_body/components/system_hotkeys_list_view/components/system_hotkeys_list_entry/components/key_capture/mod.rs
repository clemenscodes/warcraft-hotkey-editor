mod hooks;
mod logic;
mod props;
mod state;
mod style;

use tw_macro::assert_component;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_key_picker_dialog::{
    SystemKeyPickerDialog, SystemKeyPickerDialogProps,
};
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::tooltip::{
    Tooltip, TooltipProps,
};
use dioxus::prelude::*;
use hooks::use_key_capture;
pub use props::KeyCaptureProps;
assert_component!(KeyCapture);

/// The key chip for a system-hotkey list row: shows the bound key and edits it on
/// click via the system key picker.
#[component]
pub fn KeyCapture(props: KeyCaptureProps) -> Element {
    let model = use_key_capture(&props);
    let tooltip = TooltipProps::from(&model);
    let picker = SystemKeyPickerDialogProps::from(&model);
    let class = style::class(model.state);
    rsx! {
        button {
            class,
            r#type: "button",
            onclick: model.on_click,
            {model.key_label}
            Tooltip { ..tooltip }
        }
        if model.is_editing {
            SystemKeyPickerDialog { ..picker }
        }
    }
}
