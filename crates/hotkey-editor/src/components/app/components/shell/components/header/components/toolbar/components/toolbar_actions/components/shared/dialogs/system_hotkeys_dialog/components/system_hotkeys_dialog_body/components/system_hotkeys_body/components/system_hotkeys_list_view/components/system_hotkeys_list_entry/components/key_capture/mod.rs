pub mod components;
mod model;
mod presentation;
mod view;

pub use view::KeyCaptureView;

use components::key_chip::KeyChip;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_key_picker_dialog::SystemKeyPickerDialog;
use dioxus::prelude::*;
use presentation::KeyCapturePresentation;
use presentation::use_key_capture;
use model::KeyCaptureModel;
use tw_macro::assert_component;

#[component]
pub fn KeyCapture(props: KeyCaptureModel) -> Element {
    let KeyCapturePresentation {
        conflict,
        label,
        onclick,
        tooltip_text,
        tooltip_placement,
        is_editing,
        title,
        current_code,
        conflicts,
        open,
        on_pick,
        on_close,
    } = use_key_capture(&props);
    rsx! {
        KeyChip {
            conflict,
            label,
            onclick,
            tooltip_text,
            tooltip_placement,
        }
        if is_editing {
            SystemKeyPickerDialog {
                title,
                current_code,
                conflicts,
                open,
                on_pick,
                on_close,
            }
        }
    }
}

assert_component!(KeyCapture);
