mod props;
mod style;

use crate::services::focus::context::use_focus_coordinator;
use dioxus::prelude::*;
use std::rc::Rc;

use style::CLASS;
use tw_macro::assert_component;

pub use props::OverrideKeyCellProps;

assert_component!(OverrideKeyCell);

/// The hotkey-capture button shown in the override panel header (and the alt/upgrade
/// sections).
#[component]
pub fn OverrideKeyCell(props: OverrideKeyCellProps) -> Element {
    let label = props.label;
    let is_editing = props.is_editing;
    let is_special = props.is_special;
    let title = props.title;
    let is_focus_target = props.is_focus_target;
    let on_activate = props.on_activate;
    let handle_click = move |_| on_activate.call(());
    let focus = use_focus_coordinator();
    // The primary key cell registers itself as the override-key focus target while it
    // is on screen, so a keyboard tile selection can hand focus on to it by state.
    let on_mounted = move |event: Event<MountedData>| {
        if is_focus_target {
            let handle: Rc<MountedData> = event.data();
            focus.set_override_key_handle(Some(handle));
        }
    };
    use_drop(move || {
        if is_focus_target {
            focus.set_override_key_handle(None);
        }
    });
    rsx! {
        button {
            class: CLASS,
            "data-editing": is_editing,
            "data-special": is_special,
            title,
            onmounted: on_mounted,
            onclick: handle_click,
            {label}
        }
    }
}
