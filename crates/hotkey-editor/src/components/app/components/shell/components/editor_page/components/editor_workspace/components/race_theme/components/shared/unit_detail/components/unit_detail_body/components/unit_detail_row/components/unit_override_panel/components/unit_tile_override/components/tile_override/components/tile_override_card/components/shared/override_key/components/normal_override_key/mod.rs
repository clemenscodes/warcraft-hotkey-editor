mod props;
mod style;

use crate::components::app::components::shell::components::shared::editable_keycap::EditableKeycap;
use dioxus::prelude::*;
pub use props::NormalOverrideKeyProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(NormalOverrideKey);

/// The single-letter override key: the square capture button around the shared gold cap.
/// The `OverrideKey` dispatcher renders it for an ordinary letter token; it wears its
/// own square box and calls `on_activate` on click to start editing. Class
/// `.normal-override-key` is load-bearing for the end-to-end selectors.
#[component]
pub fn NormalOverrideKey(props: NormalOverrideKeyProps) -> Element {
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
