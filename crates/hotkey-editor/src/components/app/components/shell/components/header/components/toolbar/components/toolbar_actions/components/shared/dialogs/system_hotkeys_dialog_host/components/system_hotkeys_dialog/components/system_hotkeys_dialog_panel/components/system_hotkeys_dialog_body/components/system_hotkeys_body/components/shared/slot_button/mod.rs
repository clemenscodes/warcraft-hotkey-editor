mod hooks;
mod props;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog_host::components::system_hotkeys_dialog::components::system_hotkeys_dialog_panel::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_slot::SystemSlot;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_key_picker_dialog::SystemKeyPickerDialog;
use crate::components::app::components::shell::components::shared::tooltip::TooltipPlacement;
use dioxus::prelude::*;
use hooks::use_slot_button;
use props::SlotButtonProps;
use style::CLASS;
use tw_macro::assert_component;

/// A big WC3 slot for the hero-selection and control-group layouts: a framed cell
/// showing a caption and its bound key, edited on click via the system key picker.
/// The host owns the focusable button and the cell's size; the framed `SystemSlot`
/// draws the cell.
#[component]
pub fn SlotButton(props: SlotButtonProps) -> Element {
    let model = use_slot_button(&props);
    let state = model.state;
    let slot_label = model.slot_label.clone();
    let key_label = model.key_label.clone();
    let conflict = model.is_conflict;
    let tooltip_text = model.conflict_title.clone();
    let tooltip_placement = TooltipPlacement::Below;
    let dragging = false;
    let picker_title = String::from("Pick a hotkey");
    let current_code = model.current_code;
    let conflicts = model.picker_conflicts.clone();
    let picker_open = true;
    let on_pick = model.on_pick;
    let on_close = model.on_close;
    let is_editing = model.is_editing;
    let on_click = model.on_click;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            tabindex: "0",
            onclick: on_click,
            SystemSlot {
                state,
                slot_label,
                key_label,
                conflict,
                tooltip_text,
                tooltip_placement,
                dragging,
            }
        }
        if is_editing {
            SystemKeyPickerDialog {
                title: picker_title,
                current_code,
                conflicts,
                open: picker_open,
                on_pick,
                on_close,
            }
        }
    }
}

assert_component!(SlotButton);
