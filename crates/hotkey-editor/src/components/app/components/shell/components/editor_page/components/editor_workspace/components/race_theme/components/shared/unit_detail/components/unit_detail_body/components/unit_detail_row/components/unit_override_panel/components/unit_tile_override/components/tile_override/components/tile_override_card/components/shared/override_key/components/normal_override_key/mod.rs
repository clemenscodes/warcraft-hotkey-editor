mod props;
mod style;

use super::super::hooks::use_override_key_handlers;
use crate::components::app::components::shell::components::shared::editable_keycap::EditableKeycap;
use dioxus::prelude::*;
pub use props::NormalOverrideKeyProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(NormalOverrideKey);

/// The single-letter override key: the square capture button around the shared gold cap.
/// The `OverrideKey` dispatcher renders it for an ordinary letter token; it wears its
/// own square box and wires the shared focus/click/mount behaviour through
/// `use_override_key_handlers`. Class `.normal-override-key` is load-bearing for keyboard
/// navigation.
#[component]
pub fn NormalOverrideKey(props: NormalOverrideKeyProps) -> Element {
    let keycap = props.keycap;
    let title = props.title;
    let is_focus_target = props.is_focus_target;
    let on_activate = props.on_activate;
    let handlers = use_override_key_handlers(is_focus_target, on_activate);
    let on_mounted = handlers.on_mounted();
    let on_click = handlers.on_click();
    rsx! {
        button {
            class: CLASS,
            title,
            onmounted: move |event| on_mounted.call(event),
            onclick: move |event| on_click.call(event),
            EditableKeycap { ..keycap }
        }
    }
}
