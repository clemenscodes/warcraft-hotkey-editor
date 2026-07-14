mod model;
mod view;

pub use view::ConflictSlotKeyView;
mod style;

use dioxus::prelude::*;
use model::ConflictSlotKeyModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ConflictSlotKey(props: ConflictSlotKeyModel) -> Element {
    let label = props.label;
    rsx! {
        div {
            class: CLASS,
            {label}
        }
    }
}

assert_component!(ConflictSlotKey);
