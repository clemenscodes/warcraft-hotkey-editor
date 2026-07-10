mod props;
mod style;

use crate::components::app::components::shell::components::shared::editable_keycap::EditableKeycap;
use dioxus::prelude::*;
pub use props::SpecialOverrideKeyProps;
use style::CLASS;
use tw_macro::assert_component;

/// The multi-character override key: a widened capture button around the shared gold cap,
/// for special tokens (Esc, Mouse4, Mouse5) whose labels do not fit the square letter
/// box. The `OverrideKey` dispatcher renders it for a special token; it calls
/// `on_activate` on click to start editing. Class `.special-override-key` is load-bearing
/// for the end-to-end selectors.
#[component]
pub fn SpecialOverrideKey(props: SpecialOverrideKeyProps) -> Element {
    let keycap = props.keycap;
    let title = props.title;
    let on_activate = props.on_activate;
    rsx! {
        button {
            class: CLASS,
            title,
            onclick: move |_event| on_activate.call(()),
            EditableKeycap { ..keycap }
        }
    }
}

assert_component!(SpecialOverrideKey);
