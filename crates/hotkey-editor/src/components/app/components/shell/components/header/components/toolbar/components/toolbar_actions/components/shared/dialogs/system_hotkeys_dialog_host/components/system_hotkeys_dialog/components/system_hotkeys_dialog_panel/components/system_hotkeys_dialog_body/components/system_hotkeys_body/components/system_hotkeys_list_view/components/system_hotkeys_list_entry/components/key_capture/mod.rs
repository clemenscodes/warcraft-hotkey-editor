pub mod components;
mod hooks;
mod props;
mod view;

pub use view::KeyCaptureView;

use components::key_chip::KeyChip;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_key_picker_dialog::SystemKeyPickerDialog;
use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;
use dioxus::prelude::*;
use hooks::use_key_capture;
use props::KeyCaptureProps;
use tw_macro::assert_component;

/// The connected host for a system-hotkey list row: it sources the row's resolved
/// binding through `use_key_capture`, renders the presentational `KeyChip`, and
/// mounts the system key picker beneath itself while editing.
#[component]
pub fn KeyCapture(props: KeyCaptureProps) -> Element {
    let model = use_key_capture(&props);
    let conflict = model.is_conflict;
    let label = model.key_label.clone();
    let onclick = model.on_click;
    let tooltip_text = model.conflict_title.clone();
    let tooltip_placement = TooltipPlacement::Above;
    let is_editing = model.is_editing;
    let title = String::from("Pick a hotkey");
    let current_code = model.current_code;
    let conflicts = model.picker_conflicts.clone();
    let on_pick = model.on_pick;
    let on_close = model.on_close;
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
                open: true,
                on_pick,
                on_close,
            }
        }
    }
}

assert_component!(KeyCapture);
