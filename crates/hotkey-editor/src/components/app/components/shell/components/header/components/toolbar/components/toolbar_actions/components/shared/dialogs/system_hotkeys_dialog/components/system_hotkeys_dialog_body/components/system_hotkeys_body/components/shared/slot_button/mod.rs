mod model;
mod presentation;
mod view;

pub use view::SlotButtonView;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_hotkeys_dialog::components::system_hotkeys_dialog_body::components::system_hotkeys_body::components::shared::system_slot::SystemSlot;
use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::system_key_picker_dialog::SystemKeyPickerDialog;
use dioxus::prelude::*;
use presentation::SlotButtonPresentation;
use presentation::use_slot_button;
use model::SlotButtonModel;
use style::CLASS;
use tw_macro::assert_component;

/// A big WC3 slot for the hero-selection and control-group layouts: a framed cell
/// showing a caption and its bound key, edited on click via the system key picker.
/// The host owns the focusable button and the cell's size; the framed `SystemSlot`
/// draws the cell.
#[component]
pub fn SlotButton(props: SlotButtonModel) -> Element {
    let SlotButtonPresentation {
        state,
        slot_label,
        key_label,
        conflict,
        tooltip_text,
        tooltip_placement,
        dragging,
        is_editing,
        title,
        current_code,
        conflicts,
        open,
        on_click,
        on_pick,
        on_close,
    } = use_slot_button(&props);
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

assert_component!(SlotButton);
