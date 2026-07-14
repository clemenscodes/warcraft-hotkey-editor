mod model;
mod view;

pub use view::NormalOverrideKeyView;
mod style;

use crate::components::app::components::shell::components::shared::editable_keycap::EditableKeycap;
use dioxus::prelude::*;
use model::NormalOverrideKeyModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn NormalOverrideKey(props: NormalOverrideKeyModel) -> Element {
    let NormalOverrideKeyModel {
        label,
        state,
        title,
        on_activate,
    } = props;
    rsx! {
        button {
            class: CLASS,
            title,
            onclick: move |_event| on_activate.call(()),
            EditableKeycap {
                label,
                state,
            }
        }
    }
}

assert_component!(NormalOverrideKey);
