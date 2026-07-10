mod props;
mod style;

use super::super::hooks::use_override_key_handlers;
use crate::components::app::components::shell::components::shared::editable_keycap::EditableKeycap;
use dioxus::prelude::*;
pub use props::SpecialOverrideKeyProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(SpecialOverrideKey);

/// The multi-character override key: a widened capture button around the shared gold cap,
/// for special tokens (Esc, Mouse4, Mouse5) whose labels do not fit the square letter
/// box. The `OverrideKey` dispatcher renders it for a special token; it wires the shared
/// focus/click/mount behaviour through `use_override_key_handlers`. Class
/// `.special-override-key` is load-bearing for keyboard navigation.
#[component]
pub fn SpecialOverrideKey(props: SpecialOverrideKeyProps) -> Element {
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
