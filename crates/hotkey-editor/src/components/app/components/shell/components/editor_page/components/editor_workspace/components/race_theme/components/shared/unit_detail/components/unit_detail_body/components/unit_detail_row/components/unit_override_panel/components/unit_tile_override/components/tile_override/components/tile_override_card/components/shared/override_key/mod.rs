mod props;
mod style;

use crate::components::app::components::shell::components::shared::editable_keycap::{
    EditableKeycap, EditableKeycapProps,
};
use crate::services::focus::context::use_focus_coordinator;
use dioxus::prelude::*;
use std::rc::Rc;

use style::CLASS;
use tw_macro::assert_component;

pub use props::OverrideKeyProps;

assert_component!(OverrideKey);

/// The hotkey-capture button shown in the override panel header (and the alt/upgrade
/// sections). The focusable, keyboard-navigable host: it owns the box size, the special
/// token widening, focus, the mount-time focus registration, and the click handler,
/// and wraps the shared `EditableKeycap` that draws the gold cap.
#[component]
pub fn OverrideKey(props: OverrideKeyProps) -> Element {
    let is_special = props.is_special;
    let keycap = EditableKeycapProps::from(&props);
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
            "data-special": is_special,
            title,
            onmounted: on_mounted,
            onclick: handle_click,
            EditableKeycap { ..keycap }
        }
    }
}
