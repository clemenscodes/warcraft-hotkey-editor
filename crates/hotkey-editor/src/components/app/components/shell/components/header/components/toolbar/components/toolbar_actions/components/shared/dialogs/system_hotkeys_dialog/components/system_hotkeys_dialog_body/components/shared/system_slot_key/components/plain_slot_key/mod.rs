mod model;
mod view;

pub use view::PlainSlotKeyView;
mod style;

use dioxus::prelude::*;
use model::PlainSlotKeyModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PlainSlotKey(props: PlainSlotKeyModel) -> Element {
    let label = props.label;
    rsx! {
        div {
            class: CLASS,
            {label}
        }
    }
}

assert_component!(PlainSlotKey);
